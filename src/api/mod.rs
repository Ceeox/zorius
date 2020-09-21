use std::sync::Arc;

use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};

mod intern_merchandise;
mod user;

use crate::api::intern_merchandise::{InternMerchandiseMutation, InternMerchandiseQuery};
use crate::api::user::{UserMutation, UserQuery};
use crate::errors::ZoriusError;
use crate::models::{
    merchandise::intern_merchandise::{InternMerchandise, NewInternOrder},
    user::{NewUser, User},
};
use crate::Context;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn api_version() -> String {
        "0.1".to_owned()
    }

    async fn table_data(ctx: &Context) -> FieldResult<Vec<InternMerchandise>> {
        InternMerchandiseQuery::table_data(ctx).await
    }

    async fn get_order(ctx: &Context, id: String) -> FieldResult<Option<InternMerchandise>> {
        InternMerchandiseQuery::get_order(ctx, id).await
    }

    // async fn get_user(ctx: &Context, id: String) -> FieldResult<Option<User>> {
    //     UserQuery::get_user(ctx, id).await
    // }
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

    // // async fn update_user(ctx: &Context, user: User) -> FieldResult<Option<User>> {
    // //     UserMutation::update_user(ctx, user).await
    // // }
}

pub fn create_schema() -> Arc<Schema> {
    Arc::new(Schema::new(
        QueryRoot {},
        MutationRoot {},
        EmptySubscription::<Context>::new(),
    ))
}
