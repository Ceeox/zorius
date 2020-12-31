use async_graphql::{Context, Object, Result};
use bson::to_document;

use crate::models::user::User;

use super::{database, MDB_COLL_NAME_USERS};

pub struct RootMutation;

#[Object]
impl RootMutation {
    async fn do_nothing(&self, _ctx: &Context<'_>) -> &str {
        "nothing"
    }
    /*
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
    */
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        email: String,
        username: String,
        password: String,
        firstname: Option<String>,
        lastname: Option<String>,
    ) -> Result<User> {
        let user = User::new(email, username, password, firstname, lastname);
        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let doc = to_document(&user)?;
        let _ = collection.insert_one(doc.clone(), None).await?;
        Ok(user.into())
    }
    /*
    async fn update_user(
        ctx: &Context,
        user_id: UserId,
        user_update: UpdateUserQuery,
    ) -> FieldResult<UserResponse> {
        UserMutation::update_user(ctx, user_id, user_update).await
    }
    */
}
