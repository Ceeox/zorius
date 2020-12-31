use actix_web::{get, post, web::Data, HttpRequest, HttpResponse};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptySubscription, Error, Result, Schema,
};
use async_graphql_actix_web::{Request, Response};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use mongodb::Database;
//use user::{UserMutation, UserQuery};

// pub(crate) mod auth;
// pub(crate) mod intern_merchandise;
// pub(crate) mod role;
// pub(crate) mod user;
// pub(crate) mod work_account;
pub mod mutations;
pub mod querys;

pub use crate::api::{mutations::RootMutation, querys::RootQuery};
use crate::config::CONFIG;
use crate::models::user::Claim;

static MDB_COLL_NAME_INTERN: &str = "merchandise_intern";
static MDB_COLL_NAME_USERS: &str = "users";

pub type RootSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

struct MyToken(String);

#[post("/graphql")]
pub async fn graphql(
    schema: Data<RootSchema>,
    http_request: HttpRequest,
    gql_request: Request,
) -> Response {
    let token = http_request
        .headers()
        .get("authorization")
        .and_then(|value| value.to_str().map(|s| MyToken(s.to_string())).ok());
    let mut request = gql_request.into_inner();
    if let Some(token) = token {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

// Enable only when we're running in debug mode
#[cfg(debug_assertions)]
#[get("/pg")]
pub async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}

pub fn database<'a>(ctx: &'a Context<'_>) -> Result<&'a Database> {
    match ctx.data::<Database>() {
        Err(_e) => Err(Error::new("missing Database in Context!")),
        Ok(r) => Ok(r),
    }
}

pub fn is_autherized(ctx: &Context<'_>) -> Result<()> {
    let value: &MyToken = match ctx.data::<MyToken>() {
        Err(_e) => return Err(Error::new("missing token")),
        Ok(r) => r,
    };
    let _split: Vec<&str> = value.0.split("Bearer").collect();
    let token = _split[1].trim();
    let key = CONFIG.secret_key.as_bytes();
    match decode::<Claim>(
        token,
        &DecodingKey::from_secret(key),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(_token) => Ok(()),
        Err(_e) => Err(Error::new("invalid token!")),
    }
}
