use std::{io::Write, sync::Arc};

use actix_multipart::Multipart;
use actix_web::{
    get, post,
    web::{self},
    HttpResponse,
};
use bson::oid::ObjectId;
use futures::{FutureExt, StreamExt, TryStreamExt};
use juniper::{EmptySubscription, FieldResult, LookAheadMethods, RootNode};
use juniper_actix::{
    graphiql_handler as gqli_handler, graphql_handler, playground_handler as play_handler,
};
use tokio::io::AsyncWriteExt;
use user::{UserMutation, UserQuery};

pub(crate) mod auth;
pub(crate) mod intern_merchandise;
// pub(crate) mod role;
pub(crate) mod user;
// pub(crate) mod work_account;

use crate::{
    api::intern_merchandise::{InternMerchandiseMutation, InternMerchandiseQuery},
    errors::ZoriusError,
    middleware::auth::AuthorizationService,
    models::user::{NewUserQuery, UpdateUserQuery},
    API_VERSION,
};
use crate::{
    models::merchandise::intern_merchandise::{
        InternMerchandiseResponse, NewInternMerchandiseQuery, UpdateInternMerchandiseQuery,
    },
    models::user::UserResponse,
};
use crate::{models::user::UserId, Context};

pub type RootSchema = RootNode<'static, RootQuery, RootMutation, EmptySubscription<Context>>;

pub async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    ctx: web::Data<Context>,
    _: AuthorizationService,
) -> Result<HttpResponse, ZoriusError> {
    Ok(graphql_handler(&ctx.root_schema, &ctx, req, payload).await?)
}

// Enable only when we're running in debug mode
#[cfg(debug_assertions)]
#[get("/graphiql")]
pub async fn graphiql() -> Result<HttpResponse, ZoriusError> {
    Ok(gqli_handler("/graphql", None).await?)
}
// Enable only when we're running in debug mode
#[cfg(debug_assertions)]
#[get("/playground")]
pub async fn zorius_playground() -> Result<HttpResponse, ZoriusError> {
    Ok(play_handler("/graphql", None).await?)
}

#[post("api/v1/upload")]
pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, ZoriusError> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("files\\{}", filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .unwrap();
        }
    }
    Ok(HttpResponse::Ok().into())
}

pub struct RootQuery;

#[juniper::graphql_object(Context = Context)]
impl RootQuery {
    async fn api_version() -> &str {
        API_VERSION
    }

    async fn table_data(ctx: &Context) -> FieldResult<Vec<InternMerchandiseResponse>> {
        InternMerchandiseQuery::table_data(ctx).await
    }

    async fn get_order(ctx: &Context, id: ObjectId) -> FieldResult<InternMerchandiseResponse> {
        InternMerchandiseQuery::get_order(ctx, id).await
    }

    async fn get_user(ctx: &Context, user_id: UserId) -> FieldResult<UserResponse> {
        UserQuery::get_user(ctx, user_id).await
    }

    async fn get_users(ctx: &Context, user_ids: Vec<UserId>) -> FieldResult<Vec<UserResponse>> {
        UserQuery::get_users(ctx, user_ids).await
    }

    async fn list_users(executor: &Executor, ctx: &Context) -> FieldResult<Vec<UserResponse>> {
        let field_name = executor.look_ahead().field_name();
        println!("{}", field_name);
        UserQuery::list_users(ctx).await
    }
}

pub struct RootMutation;

#[juniper::graphql_object(Context = Context)]
impl RootMutation {
    async fn new_intern_order(
        ctx: &Context,
        new_intern_order: NewInternMerchandiseQuery,
    ) -> FieldResult<InternMerchandiseResponse> {
        InternMerchandiseMutation::new_intern_order(ctx, new_intern_order).await
    }

    async fn update_intern_order(
        ctx: &Context,
        order_id: ObjectId,
        inter_update: UpdateInternMerchandiseQuery,
    ) -> FieldResult<InternMerchandiseResponse> {
        InternMerchandiseMutation::update_intern_order(ctx, order_id, inter_update).await
    }

    async fn create_user(ctx: &Context, new_user: NewUserQuery) -> FieldResult<UserResponse> {
        UserMutation::new_user(ctx, new_user).await
    }

    async fn update_user(
        ctx: &Context,
        user_id: UserId,
        user_update: UpdateUserQuery,
    ) -> FieldResult<UserResponse> {
        UserMutation::update_user(ctx, user_id, user_update).await
    }
}

pub fn create_schema() -> Arc<RootSchema> {
    Arc::new(RootSchema::new(
        RootQuery {},
        RootMutation {},
        EmptySubscription::<Context>::new(),
    ))
}
