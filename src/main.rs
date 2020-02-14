use bson::doc;
use mongodb::{options::ClientOptions, Client};

use std::io;
use std::sync::Arc;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod merchandise;
mod config;

use crate::merchandise::{Context, Schema};
use crate::config::ZoriusConfig;

const GRAPH_QL_URL: &str = "http://localhost:8080/graphql";
const MONGODB_URL: &str = "mongodb://localhost:27017";

#[post("/graphql")]
async fn graphql(
    st: web::Data<Arc<Schema>>,
    db: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &db);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

// Enable only when we're running in debug mode
#[cfg(debug_assertions)]
#[get("/graphql")]
async fn graphiql() -> HttpResponse {
    let html = graphiql_source(GRAPH_QL_URL);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = config::ZoriusConfig::new()?;
    let dbconf = config.db_config;
    
    // create mongodb connection
    let url = format!("mongodb+srv://{}:{}@{}/{}", dbconf.username, dbconf.password, dbconf.server_domain, dbconf.db_name);
    let mut client_options = ClientOptions::parse(&url).unwrap();
    client_options.app_name = Some(dbconf.application_name);
    let client = Client::with_options(client_options).unwrap();
    let ctx = Context { client };

    // Create Juniper schema
    let schema = std::sync::Arc::new(merchandise::create_schema());

    // Start http server
    let webserver_url = format!("{}:{}", config.web_config.ip, config.web_config.port);
    HttpServer::new(move || {
        App::new()
            .data(ctx.clone())
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(graphql)
            .service(graphiql)
    })
    .bind(webserver_url)?
    .run()
    .await
}
