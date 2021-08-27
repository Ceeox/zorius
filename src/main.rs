use std::io::BufReader;
use std::{fs::File, time::Duration};

use actix_cors::Cors;

use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    http::Method,
    middleware::{DefaultHeaders, Logger},
    App, HttpServer,
};
use async_graphql::{EmptySubscription, Schema};
use errors::ZoriusError;
use log::{debug, error, info};
use models::{roles::RoleCache, upload::Storage};
use rustls::{
    internal::pemfile::certs, internal::pemfile::pkcs8_private_keys, NoClientAuth, ServerConfig,
};
use sqlx::PgPool;
use tokio::time::sleep;
use uuid::Uuid;

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

    Ok(Database::new(PgPool::connect(&url).await?).await)
}

fn setup_log() {
    let value = format!("{},actix_web={}", CONFIG.log_level, CONFIG.log_level);
    std::env::set_var("RUST_LOG", &value);
    debug!("Running in DEBUG mode");

    env_logger::init();
}

fn setup_tls() -> ServerConfig {
    let mut tls_config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open(CONFIG.web.cert_path.clone().unwrap()).unwrap());
    let key_file = &mut BufReader::new(File::open(CONFIG.web.key_path.clone().unwrap()).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    tls_config
        .set_single_cert(cert_chain, keys.remove(0))
        .unwrap();
    tls_config
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

    let role_cache = RoleCache::new();

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
        .data(role_cache)
        .data(Storage::default())
        .finish();

    // Start http server
    let webserver_url = format!("{}:{}", CONFIG.web.ip, CONFIG.web.port);
    let log_format = CONFIG.web.log_format.clone();
    let gov_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(10)
        .finish()
        .unwrap();

    let http_server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allowed_methods(&[Method::GET, Method::POST, Method::OPTIONS])
                    .allowed_origin("localhost")
                    .allowed_origin(&CONFIG.domain),
            )
            .wrap(DefaultHeaders::new().header("x-request-id", Uuid::new_v4().to_string()))
            .wrap(Logger::new(&log_format))
            //.wrap(Governor::new(&gov_conf))
            .service(graphql)
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
