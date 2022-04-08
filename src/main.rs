use std::io::BufReader;
use std::iter;
use std::{fs::File, time::Duration};

use actix_cors::Cors;
use actix_files::Files;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::guard;
use actix_web::http::header;
use actix_web::web::{self, Data};
use actix_web::{
    http::Method,
    middleware::{DefaultHeaders, Logger},
    App, HttpServer,
};
use async_graphql::http::MultipartOptions;
use async_graphql::Schema;
use log::{debug, error, info};
use migration::{Migrator, MigratorTrait};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{read_one, Item};
use sea_orm::{Database as SeaOrmDatabase, DatabaseConnection};
use tokio::time::sleep;
use uuid::Uuid;

use crate::api::{graphql_ws, Subscription};
use crate::errors::Error;

mod api;
mod claim;
mod config;
mod customer;
mod errors;
mod guards;
mod mailer;
mod project;
mod simple_broker;
mod upload;
mod user;
mod validators;
mod work_report;

use crate::{
    api::{graphql, playground, Mutation, Query},
    config::CONFIG,
};

const API_VERSION: &str = "v1";

async fn setup_pg() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        CONFIG.db.username, CONFIG.db.password, CONFIG.db.host, CONFIG.db.port, CONFIG.db.name
    );
    let pw_hidden_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        CONFIG.db.username, "<hidden>", CONFIG.db.host, CONFIG.db.port, CONFIG.db.name
    );
    info!("Connecting to: {:?}", pw_hidden_url);

    SeaOrmDatabase::connect(&url).await
}

fn setup_log() {
    let value = format!("{},actix_web={}", CONFIG.log_level, CONFIG.log_level);
    std::env::set_var("RUST_LOG", &value);
    debug!("Running in DEBUG mode");

    env_logger::init();
}

fn setup_tls() -> ServerConfig {
    let mut certs = Vec::new();
    let mut cert_file = BufReader::new(File::open(CONFIG.web.cert_path.clone().unwrap()).unwrap());
    for item in iter::from_fn(|| read_one(&mut cert_file).transpose()).flatten() {
        if let Item::X509Certificate(cert) = item {
            certs.push(Certificate(cert))
        }
    }

    let mut key_file = BufReader::new(File::open(CONFIG.web.key_path.clone().unwrap()).unwrap());
    let key_buf = match read_one(&mut key_file).unwrap().unwrap() {
        Item::PKCS8Key(key) => key,
        Item::RSAKey(key) => key,
        _ => vec![],
    };

    let key = PrivateKey(key_buf);
    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .unwrap()
}

fn check_folders() -> Result<(), Error> {
    use std::path::Path;
    if !Path::new("files").exists() {
        std::fs::create_dir("files")?;
    }
    if !Path::new("static").exists() {
        std::fs::create_dir("static/avatar")?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
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

    info!("running migrations");
    Migrator::up(&database, None)
        .await
        .expect("migrations failed");

    let schema = Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(database)
    .finish();

    // Start http server
    let webserver_url = format!("{}:{}", CONFIG.web.ip, CONFIG.web.port);

    let url = match CONFIG.web.enable_ssl {
        true => format!("https://{}:{}", CONFIG.domain, CONFIG.web.port),
        false => format!("http://{}:{}", CONFIG.domain, CONFIG.web.port),
    };

    let log_format = CONFIG.web.log_format.clone();

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(100)
        .finish()
        .unwrap();

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
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
            .wrap(Governor::new(&governor_conf))
            .wrap(DefaultHeaders::new().add(("x-request-id", Uuid::new_v4().to_string())))
            .wrap(Logger::new(&log_format))
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql)
                    .app_data(MultipartOptions::default().max_num_files(5)),
            )
            .service(
                web::resource("/playground")
                    .guard(guard::Get())
                    .to(playground),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(graphql_ws),
            )
            .service(
                Files::new("/", "static")
                    .index_file("index.html")
                    .prefer_utf8(true),
            )
            .service(Files::new("/avatar", "./static/avatar").prefer_utf8(true))
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
