use std::sync::Arc;

use bson::oid::ObjectId;
use juniper::{EmptySubscription, FieldResult, RootNode};
use user::{UserMutation, UserQuery};

mod intern_merchandise;
mod user;

use crate::{
    models::merchandise::intern_merchandise::InternMerchandiseUpdate, models::user::NewUser,
    models::user::UserUpdate,
};

use crate::models::{
    merchandise::intern_merchandise::{InternMerchandise, NewInternOrder},
    user::User,
};
use crate::Context;
use crate::{
    api::intern_merchandise::{InternMerchandiseMutation, InternMerchandiseQuery},
    models::merchandise::intern_merchandise::InternMerchandiseList,
};

pub type RootSchema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn api_version() -> String {
        VERSION.to_owned()
    }

    async fn table_data(ctx: &Context) -> FieldResult<InternMerchandiseList> {
        InternMerchandiseQuery::table_data(ctx).await
    }

    async fn get_order(ctx: &Context, id: ObjectId) -> FieldResult<InternMerchandise> {
        InternMerchandiseQuery::get_order(ctx, id).await
    }
    async fn get_user(ctx: &Context, user_id: ObjectId) -> FieldResult<User> {
        UserQuery::get_user(ctx, user_id).await
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn new_intern_order(
        ctx: &Context,
        new_intern_order: NewInternOrder,
    ) -> FieldResult<InternMerchandise> {
        InternMerchandiseMutation::new_intern_order(ctx, new_intern_order).await
    }
    async fn update_intern_order(
        ctx: &Context,
        order_id: ObjectId,
        inter_update: InternMerchandiseUpdate,
    ) -> FieldResult<InternMerchandise> {
        InternMerchandiseMutation::update_intern_order(ctx, order_id, inter_update).await
    }

    async fn new_user(ctx: &Context, new_user: NewUser) -> FieldResult<User> {
        UserMutation::new_user(ctx, new_user).await
    }

    async fn update_user(
        ctx: &Context,
        user_id: ObjectId,
        user_update: UserUpdate,
    ) -> FieldResult<User> {
        UserMutation::update_user(ctx, user_id, user_update).await
    }
}

pub fn create_schema() -> Arc<RootSchema> {
    Arc::new(RootSchema::new(
        QueryRoot {},
        MutationRoot {},
        EmptySubscription::<Context>::new(),
    ))
}
