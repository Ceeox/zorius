use bson::doc;
use mongodb::{options::ClientOptions, Client};

use std::io;
use std::sync::Arc;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer, http::header};
use actix_files::Files;
use actix_cors::Cors;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use dotenv::dotenv;
use dotenv_codegen::dotenv;

mod merchandise;
mod config;

use crate::merchandise::{Context, Schema};
use crate::config::ZoriusConfig;

const GRAPH_QL_URL: &str = "http://localhost:8080/graphql";

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
#[get("/graphiql")]
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
    dotenv().ok();
    
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
            // .wrap(
            //     Cors::new() // <- Construct CORS middleware builder
            //     .allowed_origin("http://localhost:8080/")
            //     .allowed_methods(vec!["GET", "POST"])
            //     .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            //     .allowed_header(header::CONTENT_TYPE)
            //     .max_age(3600)
            //     .finish()
            // )                
            .data(ctx.clone())
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(graphql)
            .service(graphiql)
            .service(Files::new("/static", "/"))
    })
    .bind(webserver_url)?
    .run()
    .await
}
