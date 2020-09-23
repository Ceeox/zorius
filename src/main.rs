use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use actix_web::{
    get,
    http::ContentEncoding,
    middleware::{Compress, DefaultHeaders, Logger},
    web, App, Error, HttpResponse, HttpServer,
};
use juniper_actix::{
    graphiql_handler as gqli_handler, graphql_handler, playground_handler as play_handler,
};
use mongodb::{options::ClientOptions, Client};
use rustls::{
    internal::pemfile::certs, internal::pemfile::pkcs8_private_keys, NoClientAuth, ServerConfig,
};
use uuid::Uuid;

mod api;
mod config;
mod errors;
mod models;

use crate::api::{create_schema, Schema};

#[derive(Clone)]
pub struct Context {
    pub client: mongodb::Client,
    pub db: mongodb::Database,
    pub schema: Arc<Schema>,
}

impl juniper::Context for Context {}

async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    ctx: web::Data<Context>,
) -> Result<HttpResponse, Error> {
    graphql_handler(&ctx.schema, &ctx, req, payload).await
}

// Enable only when we're running in debug mode
#[cfg(debug_assertions)]
#[get("/graphiql")]
async fn graphiql() -> Result<HttpResponse, Error> {
    gqli_handler("/graphql", None).await
}

// Enable only when we're running in debug mode
#[cfg(debug_assertions)]
#[get("/playground")]
async fn zorius_playground() -> Result<HttpResponse, Error> {
    play_handler("/graphql", None).await
}

#[actix_web::main]
async fn main() -> Result<(), errors::ZoriusError> {
    if cfg!(debug_accertions) {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    } else {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let config = config::Config::new()?;

    // create mongodb connection
    let url = format!(
        "mongodb+srv://{}:{}@{}/{}",
        config.db_config.username,
        config.db_config.password,
        config.db_config.server_domain,
        config.db_config.db_name
    );
    let mut client_options = ClientOptions::parse(&url).await?;
    client_options.app_name = Some(config.db_config.app_name);
    let client = Client::with_options(client_options)?;
    let db = client.database("zorius");
    // Create Juniper schema
    let schema = create_schema();
    let ctx = Context { client, db, schema };

    let mut tls_config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    tls_config
        .set_single_cert(cert_chain, keys.remove(0))
        .unwrap();

    // Start http server
    let webserver_url = format!("{}:{}", config.web_config.ip, config.web_config.port);
    HttpServer::new(move || {
        App::new()
            .data(ctx.clone())
            .wrap(DefaultHeaders::new().header("x-request-id", Uuid::new_v4().to_string()))
            .wrap(Logger::new("IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s DURATION: %D X-REQUEST-ID:%{x-request-id}o"))
            .wrap(Compress::new(ContentEncoding::Br))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql)),
            )
            .service(graphiql)
            .service(zorius_playground)
    })
    .bind_rustls(webserver_url, tls_config)?
    .run().await?;

    Ok(())
}
