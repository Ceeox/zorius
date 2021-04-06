use actix_web::{get, post, web::Data, HttpRequest, HttpResponse};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptySubscription, Error, MergedObject, Object, Result, Schema,
};
use async_graphql_actix_web::{Request, Response};
use mongodb::Database;

pub mod claim;
pub mod company;
pub mod customer;
pub mod intern_merchandise;
pub mod project;
pub mod role;
pub mod user;

use crate::{config::CONFIG, API_VERSION};

use self::{
    claim::Token,
    customer::{CustomerMutation, CustomerQuery},
    intern_merchandise::{InternMerchandiseMutation, InternMerchandiseQuery},
    project::{ProjectMutation, ProjectQuery},
    role::{RoleMutation, RoleQuery},
    user::{UserMutation, UserQuery},
};

pub(crate) static MDB_COLL_NAME_USERS: &str = "users";
pub(crate) static MDB_COLL_WORK_ACCOUNTS: &str = "workaccounts";
pub(crate) static MDB_COLL_WORK_REPORTS: &str = "work_reports";
pub(crate) static MDB_COLL_INTERN_MERCH: &str = "merchandise_intern";
pub(crate) static MDB_COLL_ROLES: &str = "roles";

pub type RootSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct Query(
    ServerQuery,
    UserQuery,
    RoleQuery,
    CustomerQuery,
    ProjectQuery,
    InternMerchandiseQuery,
);

#[derive(Default, MergedObject)]
pub struct Mutation(
    UserMutation,
    RoleMutation,
    CustomerMutation,
    ProjectMutation,
    InternMerchandiseMutation,
);

#[derive(Default)]
pub struct ServerQuery;

#[Object]
impl ServerQuery {
    async fn api_version(&self) -> &str {
        API_VERSION
    }

    async fn registration_enabled(&self) -> bool {
        CONFIG.registration_enabled
    }
}

#[post("/graphql")]
pub async fn graphql(
    schema: Data<RootSchema>,
    http_request: HttpRequest,
    gql_request: Request,
) -> Response {
    let token = http_request
        .headers()
        .get("authorization")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok());
    let mut request = gql_request.into_inner();
    if let Some(token) = token {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

// Enable only when we're running in debug mode
#[get("/playground")]
pub async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

pub fn database<'a>(ctx: &'a Context<'_>) -> Result<&'a Database> {
    match ctx.data::<Database>() {
        Err(_e) => Err(Error::new("missing Database in Context!")),
        Ok(r) => Ok(r),
    }
}
