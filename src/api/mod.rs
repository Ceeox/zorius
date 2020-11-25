use std::sync::Arc;

use bson::oid::ObjectId;
use juniper::{EmptySubscription, FieldResult, RootNode};
use user::{UserMutation, UserQuery};

mod intern_merchandise;
mod user;
mod role;
mod time_recording;

use crate::Context;
use crate::{
    api::intern_merchandise::{InternMerchandiseMutation, InternMerchandiseQuery},
    models::user::{NewUserQuery, UpdateUserQuery},
};
use crate::{
    models::merchandise::intern_merchandise::{
        InternMerchandise, InternMerchandiseResponse, NewInternMerchandiseQuery,
        UpdateInternMerchandiseQuery,
    },
    models::user::UserResponse,
};

pub type RootSchema = RootNode<'static, RootQuery, RootMutation, EmptySubscription<Context>>;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct RootQuery;

#[juniper::graphql_object(Context = Context)]
impl RootQuery {
    async fn api_version() -> String {
        VERSION.to_owned()
    }

    async fn table_data(ctx: &Context) -> FieldResult<Vec<InternMerchandiseResponse>> {
        InternMerchandiseQuery::table_data(ctx).await
    }

    async fn get_order(ctx: &Context, id: ObjectId) -> FieldResult<InternMerchandiseResponse> {
        InternMerchandiseQuery::get_order(ctx, id).await
    }

    async fn get_user(ctx: &Context, user_id: ObjectId) -> FieldResult<UserResponse> {
        UserQuery::get_user(ctx, user_id).await
    }

    async fn get_users(ctx: &Context, user_ids: Vec<ObjectId>) -> FieldResult<Vec<UserResponse>> {
        UserQuery::get_users(ctx, user_ids).await
    }

    async fn list_users(ctx: &Context) -> FieldResult<Vec<UserResponse>> {
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

    async fn new_user(ctx: &Context, new_user: NewUserQuery) -> FieldResult<UserResponse> {
        UserMutation::new_user(ctx, new_user).await
    }

    async fn update_user(
        ctx: &Context,
        user_id: ObjectId,
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
