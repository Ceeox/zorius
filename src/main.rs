use std::io::BufReader;
use std::{fs::File, time::Duration};

use actix_cors::Cors;
use actix_files::Files;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::{
    http::ContentEncoding,
    middleware::{Compress, DefaultHeaders, Logger},
    App, HttpServer,
};
use async_graphql::{EmptySubscription, Schema};
use mongodb::{options::ClientOptions, options::ResolverConfig, Client};
use rustls::{
    internal::pemfile::certs, internal::pemfile::pkcs8_private_keys, NoClientAuth, ServerConfig,
};

use api::{gql_playgound, Mutation, Query};
use errors::ZoriusError;
use models::{roles::RoleCache, upload::Storage};
use uuid::Uuid;

mod api;
mod config;
mod database;
mod errors;
mod helper;
mod mailer;
mod models;

use crate::{api::graphql, config::CONFIG, database::Database};

const API_VERSION: &str = "v1";

async fn setup_mongodb() -> Result<Client, ZoriusError> {
    let url = format!(
        "mongodb+srv://{}:{}@{}/{}",
        CONFIG.db.username, CONFIG.db.password, CONFIG.db.server, CONFIG.db.name
    );

    // Use to cloudflare resolver to work around a mongodb dns resolver issue.
    // For more Infos: https://github.com/mongodb/mongo-rust-driver#windows-dns-note
    let mut client_options =
        ClientOptions::parse_with_resolver_config(&url, ResolverConfig::cloudflare()).await?;

    client_options.app_name = Some(CONFIG.db.app_name.clone());
    Ok(Client::with_options(client_options)?)
}

fn setup_log() {
    if CONFIG.debug {
        std::env::set_var("RUST_LOG", "actix_web=debug");
        println!("Running in DEBUG MODE...");
    } else {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

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
    /*
        if !Path::new("static").exists() {
            panic!("missing frondend files folder");
        }
    */

    if !Path::new("files").exists() {
        std::fs::create_dir("files")?;
    }
    Ok(())
}

#[actix_web::main]
async fn main() -> Result<(), errors::ZoriusError> {
    setup_log();
    check_folders()?;

    let client = setup_mongodb().await?;
    let role_cache = RoleCache::new();
    let db = client.database(&CONFIG.db.name);
    let database = Database::new(client, db.clone());

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(database)
        .data(db)
        .data(role_cache)
        .data(Storage::default())
        .finish();

    // Start http server
    let webserver_url = format!("{}:{}", CONFIG.web.ip, CONFIG.web.port);
    let log_format = CONFIG.web.log_format.clone();
    let store = MemoryStore::new();
    let http_server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(Cors::permissive())
            .wrap(DefaultHeaders::new().header("x-request-id", Uuid::new_v4().to_string()))
            .wrap(Logger::new(&log_format))
            .wrap(Compress::new(ContentEncoding::Auto))
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
                    .with_interval(Duration::from_secs(60))
                    .with_max_requests(50),
            )
            // graphql api
            .service(graphql)
            .service(gql_playgound)
            // static file serving
            .service(
                Files::new("/", "static")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
            .service(
                Files::new("/files", "./files")
                    .prefer_utf8(true)
                    .show_files_listing(),
            )
    });

    let res = if CONFIG.web.enable_ssl {
        let tls_config = setup_tls();

        http_server
            .bind_rustls(webserver_url, tls_config)?
            .run()
            .await?
    } else {
        http_server.bind(webserver_url)?.run().await?
    };
    Ok(res)
}
