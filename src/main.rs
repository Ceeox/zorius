use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use actix_web::{
    get,
    http::ContentEncoding,
    middleware::{Compress, DefaultHeaders, Logger},
    web, App, Error, HttpResponse, HttpServer,
};
use config::Config;
use errors::ZoriusError;
use juniper_actix::{
    graphiql_handler as gqli_handler, graphql_handler, playground_handler as play_handler,
};
use mongodb::{options::ClientOptions, options::ResolverConfig, Client};
use rustls::{
    internal::pemfile::certs, internal::pemfile::pkcs8_private_keys, NoClientAuth, ServerConfig,
};
use uuid::Uuid;

mod api;
mod config;
mod errors;
mod models;

use crate::api::{create_schema, RootSchema};

const API_VERSION: &str = "v1";
const API_BASE_URL: &str = "graphql";

#[derive(Clone)]
pub struct Context {
    pub client: mongodb::Client,
    pub db: mongodb::Database,
    pub root_schema: Arc<RootSchema>,
}

impl juniper::Context for Context {}

async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    ctx: web::Data<Context>,
) -> Result<HttpResponse, Error> {
    graphql_handler(&ctx.root_schema, &ctx, req, payload).await
}

// Enable only when we're running in debug mode
// #[cfg(debug_assertions)]
#[get("/graphiql")]
async fn graphiql() -> Result<HttpResponse, Error> {
    gqli_handler("/graphql", None).await
}

// Enable only when we're running in debug mode
// #[cfg(debug_assertions)]
#[get("/playground")]
async fn zorius_playground() -> Result<HttpResponse, Error> {
    play_handler("/graphql", None).await
}

async fn setup_mongo(config: &Config) -> Result<Client, ZoriusError> {
    let url = format!(
        "mongodb+srv://{}:{}@{}/{}",
        config.db_config.username,
        config.db_config.password,
        config.db_config.server_domain,
        config.db_config.db_name
    );
    // Use to cloudflare resolver to work around a mongodb dns resolver issue.
    // For more Infos: https://github.com/mongodb/mongo-rust-driver#windows-dns-note
    let mut client_options = if cfg!(windows) {
        ClientOptions::parse_with_resolver_config(&url, ResolverConfig::cloudflare()).await?
    } else {
        ClientOptions::parse(&url).await?
    };

    client_options.app_name = Some(config.db_config.app_name.clone());
    Ok(Client::with_options(client_options)?)
}

fn setup_log() {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    } else {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
}

fn setup_tls(config: &Config) -> ServerConfig {
    let mut tls_config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open(&config.web_config.cert_path).unwrap());
    let key_file = &mut BufReader::new(File::open(&config.web_config.key_path).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    tls_config
        .set_single_cert(cert_chain, keys.remove(0))
        .unwrap();
    tls_config
}

#[actix_web::main]
async fn main() -> Result<(), errors::ZoriusError> {
    setup_log();

    let config = Config::new()?;

    let client = setup_mongo(&config).await?;
    let db = client.database(&config.db_config.db_name);

    // Create Juniper schema
    let root_schema = create_schema();

    let ctx = Context {
        client,
        db,
        root_schema,
    };

    let tls_config = setup_tls(&config);

    // Start http server
    let webserver_url = format!("{}:{}", config.web_config.ip, config.web_config.port);
    let log_format = config.web_config.log_format.clone();
    HttpServer::new(move || {
        App::new()
            .data(ctx.clone())
            .wrap(DefaultHeaders::new().header("x-request-id", Uuid::new_v4().to_string()))
            .wrap(Logger::new(&log_format))
            .wrap(Compress::new(ContentEncoding::Br))
            .service(
                web::resource(&format!("{}/{}", API_VERSION, API_BASE_URL))
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql)),
            )
            .service(graphiql)
            .service(zorius_playground)
        // TODO: add service for frontend files
        // TODO: add service for static files
    })
    .bind_rustls(webserver_url, tls_config)?
    .run()
    .await?;

    Ok(())
}
