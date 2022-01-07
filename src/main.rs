use std::io::BufReader;
use std::{fs::File, time::Duration};

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{
    http::Method,
    middleware::{DefaultHeaders, Logger},
    App, HttpServer,
};
use async_graphql::{EmptySubscription, Schema};
use log::{debug, error, info};
use rustls::PrivateKey;
use rustls::{internal::msgs::codec::Codec, server::NoClientAuth, Certificate, ServerConfig};
use sea_orm::Database as SeaOrmDatabase;
use sqlx::PgPool;
use tokio::time::sleep;
use uuid::Uuid;

use crate::{errors::ZoriusError, models::upload::Storage};

mod api;
mod config;
mod database;
mod errors;
mod helper;
mod mailer;
mod models;
mod validators;
mod view;

use crate::{
    api::{graphql, playground, Mutation, Query},
    config::CONFIG,
    database::Database,
};

const API_VERSION: &str = "v1";

async fn setup_pg() -> Result<Database, sqlx::Error> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        CONFIG.db.username, CONFIG.db.password, CONFIG.db.server, CONFIG.db.port, CONFIG.db.name
    );
    let pw_hidden_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        CONFIG.db.username, "<hidden>", CONFIG.db.server, CONFIG.db.port, CONFIG.db.name
    );
    info!("Connecting to: {:?}", pw_hidden_url);

    let db = SeaOrmDatabase::connect(&url).await.unwrap();

    Ok(Database::new(PgPool::connect(&url).await?, db).await)
}

fn setup_log() {
    let value = format!("{},actix_web={}", CONFIG.log_level, CONFIG.log_level);
    std::env::set_var("RUST_LOG", &value);
    debug!("Running in DEBUG mode");

    env_logger::init();
}

fn setup_tls() -> ServerConfig {
    let cert_file = &mut BufReader::new(File::open(CONFIG.web.cert_path.clone().unwrap()).unwrap());
    let key_file = &mut BufReader::new(File::open(CONFIG.web.key_path.clone().unwrap()).unwrap());
    let cert_chain = Certificate::read_bytes(cert_file.buffer()).unwrap();
    let key = PrivateKey(key_file.buffer().to_owned());
    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![cert_chain], key)
        .unwrap()
}

fn check_folders() -> Result<(), ZoriusError> {
    use std::path::Path;
    if !Path::new("files").exists() {
        std::fs::create_dir("files")?;
    }
    Ok(())
}

#[actix_web::main]
async fn main() -> Result<(), errors::ZoriusError> {
    setup_log();
    check_folders()?;

    let mut db_connect_trys: i32 = 1;
    let database = loop {
        match setup_pg().await {
            Ok(r) => break r,
            Err(e) => {
                error!(
                    "Failed to connect to postgres database (Try: {}).\n{}",
                    db_connect_trys, e
                );
                db_connect_trys += 1;
                sleep(Duration::from_secs(10)).await;
                continue;
            }
        }
    };
    info!("Successfully connected to database");

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(database)
        .data(Storage::default())
        .finish();

    // Start http server
    let webserver_url = format!("{}:{}", CONFIG.web.ip, CONFIG.web.port);

    let url = match (CONFIG.web.enable_ssl, CONFIG.debug) {
        (true, true) => format!("https://localhost:{}", CONFIG.web.port),
        (true, false) => format!("https://{}:{}", CONFIG.domain, CONFIG.web.port),
        (false, true) => format!("http://localhost:{}", CONFIG.web.port),
        (false, false) => format!("http://{}:{}", CONFIG.domain, CONFIG.web.port),
    };

    let log_format = CONFIG.web.log_format.clone();

    let http_server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(
                Cors::default()
                    .allowed_methods(&[Method::GET, Method::POST, Method::OPTIONS])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .allowed_origin(&url)
                    .allowed_origin("http://localhost:4200")
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(DefaultHeaders::new().header("x-request-id", Uuid::new_v4().to_string()))
            .wrap(Logger::new(&log_format))
            .service(graphql)
            .app_data(schema.clone())
            .service(playground)
    });

    let res = if CONFIG.web.enable_ssl {
        let tls_config = setup_tls();
        info!("Starting webserver...");
        http_server
            .bind_rustls(webserver_url, tls_config)?
            .run()
            .await?
    } else {
        http_server.bind(webserver_url)?.run().await?
    };
    Ok(res)
}
