use actix_web::{
    web::{self, Data},
    HttpRequest, HttpResponse,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, Enum, ErrorExtensions, MergedObject, MergedSubscription, Object, Result, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use serde::Deserialize;

use crate::{
    claim::Token,
    config::CONFIG,
    customer::{CustomerMutation, CustomerQuery, CustomerSubscription},
    errors::Error,
    project::{ProjectMutation, ProjectQuery, ProjectSubscription},
    user::{UserMutation, UserQuery, UserSubscription},
    work_report::{WorkReportMutation, WorkReportQuery, WorkReportSubscription},
    API_VERSION,
};

pub type RootSchema = Schema<Query, Mutation, Subscription>;

#[derive(MergedObject, Default)]
pub struct Query(
    ServerQuery,
    UserQuery,
    CustomerQuery,
    ProjectQuery,
    WorkReportQuery,
);

#[derive(Default, MergedObject)]
pub struct Mutation(
    UserMutation,
    CustomerMutation,
    ProjectMutation,
    WorkReportMutation,
);

#[derive(Default, MergedSubscription)]
pub struct Subscription(
    UserSubscription,
    ProjectSubscription,
    CustomerSubscription,
    WorkReportSubscription,
);

#[derive(Enum, Eq, PartialEq, Copy, Clone)]
pub enum MutationType {
    Created,
    Deleted,
    Updated,
}

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

pub async fn graphql(
    schema: Data<RootSchema>,
    http_request: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let token = http_request
        .headers()
        .get("authorization")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok());
    let mut request = gql_request.into_inner();
    if let Some(token) = token {
        request = request.data(token);
    }

    let conn_info = http_request.connection_info().clone();
    request = request.data(conn_info);

    schema.execute(request).await.into()
}

pub async fn graphql_ws(
    schema: Data<RootSchema>,
    http_request: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let token = http_request
        .headers()
        .get("authorization")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok());
    let mut data = async_graphql::Data::default();
    if let Some(token) = token {
        data.insert(token);
    }
    Ok(GraphQLSubscription::new(Schema::clone(&*schema))
        .with_data(data)
        .on_connection_init(on_connection_init)
        .start(&http_request, payload)?)
}

pub async fn playground() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}

pub async fn on_connection_init(
    value: serde_json::Value,
) -> Result<async_graphql::Data, async_graphql::Error> {
    #[derive(Deserialize)]
    struct Payload {
        authorization: String,
    }

    if let Ok(payload) = serde_json::from_value::<Payload>(value) {
        let mut data = async_graphql::Data::default();
        data.insert(Token(payload.authorization));
        Ok(data)
    } else {
        Err(Error::MissingToken.extend())
    }
}

pub fn database<'a>(ctx: &'a Context<'_>) -> Result<&'a sea_orm::DatabaseConnection, Error> {
    match ctx.data::<sea_orm::DatabaseConnection>() {
        Err(_e) => Err(Error::MissingDatabase),
        Ok(r) => Ok(r),
    }
}
