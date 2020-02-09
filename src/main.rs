use bson::{bson, doc};
use mongodb::{options::ClientOptions, Client};

use std::io;
use std::sync::{Arc, Mutex};

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod merchandise;

use crate::merchandise::{Context, Schema};

const GRAPH_QL_URL: &str = "http://127.0.0.1:8080/graphql";
const MONGODB_URL: &str = "mongodb://localhost:27017";

// fn main() {
//     let mut client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
//     client_options.app_name = Some("test".to_owned());
//     let client = Client::with_options(client_options).unwrap();
//     for db_name in client.list_databases(None).unwrap() {
//         println!("{}", db_name);
//     }
//     // let db = client.database("test2");
//     // let collection = db.collection("test2");

//     // let docs = vec![
//     //     doc! { "title": "1984", "author": "George Orwell" },
//     //     doc! { "title": "Animal Farm", "author": "George Orwell" },
//     //     doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
//     // ];
//     // // Insert some documents into the "mydb.books" collection.
//     // collection.insert_many(docs, None).unwrap();
// }

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

    // create mongodb connection
    let mut client_options = ClientOptions::parse(MONGODB_URL).unwrap();
    client_options.app_name = Some("zorius".to_owned());
    let client = Client::with_options(client_options).unwrap();
    let ctx = Context { client };

    // Create Juniper schema
    let schema = std::sync::Arc::new(merchandise::create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(ctx.clone())
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
