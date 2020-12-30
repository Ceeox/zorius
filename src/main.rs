use std::io::BufReader;
use std::sync::Arc;
use std::{fs::File, time::Duration};

use actix_files::Files;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::{
    http::ContentEncoding,
    middleware::{Compress, DefaultHeaders, Logger},
    web, App, HttpServer,
};
use errors::ZoriusError;
use mongodb::{options::ClientOptions, options::ResolverConfig, Client};
use rustls::{
    internal::pemfile::certs, internal::pemfile::pkcs8_private_keys, NoClientAuth, ServerConfig,
};
use uuid::Uuid;

mod api;
mod config;
mod errors;
mod helper;
mod middleware;
mod models;

use crate::{
    api::{
        auth::{login, register},
        create_schema, graphiql, graphql, upload, zorius_playground, RootSchema,
    },
    config::CONFIG,
};

const API_VERSION: &str = "v1";
const API_BASE: &str = "graphql";
const API_AUTH: &str = "auth";

#[derive(Clone)]
pub struct Context {
    pub client: mongodb::Client,
    pub db: mongodb::Database,
    pub root_schema: Arc<RootSchema>,
}

impl juniper::Context for Context {}

async fn setup_mongodb() -> Result<Client, ZoriusError> {
    let url = format!(
        "mongodb+srv://{}:{}@{}/{}",
        CONFIG.db_config.username,
        CONFIG.db_config.password,
        CONFIG.db_config.server_domain,
        CONFIG.db_config.db_name
    );

    // Use to cloudflare resolver to work around a mongodb dns resolver issue.
    // For more Infos: https://github.com/mongodb/mongo-rust-driver#windows-dns-note
    let mut client_options =
        ClientOptions::parse_with_resolver_config(&url, ResolverConfig::cloudflare()).await?;

    client_options.app_name = Some(CONFIG.db_config.app_name.clone());
    Ok(Client::with_options(client_options)?)
}

fn setup_log() {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    } else {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    println!("{:#?}", std::env::var("RUST_LOG"));
    env_logger::init();
}

fn setup_tls() -> ServerConfig {
    let mut tls_config = ServerConfig::new(NoClientAuth::new());
    let cert_file =
        &mut BufReader::new(File::open(CONFIG.web_config.cert_path.clone().unwrap()).unwrap());
    let key_file =
        &mut BufReader::new(File::open(CONFIG.web_config.key_path.clone().unwrap()).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    tls_config
        .set_single_cert(cert_chain, keys.remove(0))
        .unwrap();
    tls_config
}

fn check_folders() -> Result<(), ZoriusError> {
    use std::path::Path;

    if !Path::new("static").exists() {
        panic!("missing frondend fiels folder");
    }

    if !Path::new("assets").exists() {
        panic!("missing assets folder");
    }

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
    let db = client.database(&CONFIG.db_config.db_name);

    // Create Juniper schema
    let root_schema = create_schema();

    let ctx = Context {
        client,
        db,
        root_schema,
    };

    // Start http server
    let webserver_url = format!("{}:{}", CONFIG.web_config.ip, CONFIG.web_config.port);
    let log_format = CONFIG.web_config.log_format.clone();
    let store = MemoryStore::new();
    let http_server = HttpServer::new(move || {
        App::new()
            .data(ctx.clone())
            .wrap(DefaultHeaders::new().header("x-request-id", Uuid::new_v4().to_string()))
            .wrap(Logger::new(&log_format))
            .wrap(Compress::new(ContentEncoding::Auto))
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
                    .with_interval(Duration::from_secs(60))
                    .with_max_requests(100),
            )
            // auth api
            .service(
                web::resource(&format!("api/{}/{}", API_VERSION, API_AUTH))
                    .route(web::post().to(login))
                    .route(web::post().to(register)),
                //.route(web::post().to(reset_password)),
            )
            .service(upload)
            // graphql api
            .service(
                web::resource(&format!("api/{}/{}", API_VERSION, API_BASE))
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql)),
            )
            .service(graphiql)
            .service(zorius_playground)
            // static file serving
            .service(
                Files::new("/", "static")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
            .service(Files::new("/assets", "assets").prefer_utf8(true))
            .service(Files::new("/files", "files").prefer_utf8(true))
        // TODO: add service for frontend files
    });

    let res = if CONFIG.web_config.enable_ssl {
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
