use std::sync::Arc;

use juniper::{EmptySubscription, FieldResult, RootNode};
use user::UserMutation;

mod intern_merchandise;
mod user;

use crate::api::user::UserQuery;

use crate::models::{
    merchandise::intern_merchandise::{InternMerchandise, NewInternOrder},
    user::User,
};
use crate::Context;
use crate::{
    api::intern_merchandise::{InternMerchandiseMutation, InternMerchandiseQuery},
    models::merchandise::intern_merchandise::InternMerchandiseList,
};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn api_version() -> String {
        "0.1".to_owned()
    }

    async fn table_data(ctx: &Context) -> FieldResult<InternMerchandiseList> {
        InternMerchandiseQuery::table_data(ctx).await
    }

    async fn get_order(ctx: &Context, id: String) -> FieldResult<Option<InternMerchandise>> {
        InternMerchandiseQuery::get_order(ctx, id).await
    }

    async fn get_user(ctx: &Context, id: String) -> FieldResult<Option<User>> {
        UserQuery::get_user(ctx, id).await
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
        new_intern_order: InternMerchandise,
    ) -> FieldResult<InternMerchandise> {
        InternMerchandiseMutation::update_intern_order(ctx, new_intern_order).await
    }

    async fn update_user(ctx: &Context, user: User) -> FieldResult<Option<User>> {
        UserMutation::update_user(ctx, user).await
    }
}

pub fn create_schema() -> Arc<Schema> {
    Arc::new(Schema::new(
        QueryRoot {},
        MutationRoot {},
        EmptySubscription::<Context>::new(),
    ))
}
