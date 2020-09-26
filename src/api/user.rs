use bson::{doc, oid::ObjectId, to_document};
use juniper::{graphql_value, FieldError, FieldResult};
use mongodb::bson::from_document;
use uuid::Uuid;

use crate::models::user::{NewUser, User, UserUpdate};
use crate::Context;

static MONGO_DB_COLLECTION_NAME: &str = "users";

pub struct UserQuery;

impl UserQuery {
    pub async fn get_user(ctx: &Context, user_id: ObjectId) -> FieldResult<User> {
        let collection = ctx.db.collection(MONGO_DB_COLLECTION_NAME);
        let filter = doc! { "_id": user_id };
        match collection.find_one(filter, None).await? {
            None => {
                return Err(FieldError::new(
                    "specified user not found",
                    graphql_value!({ "error": "specified user not found" }),
                ))
            }
            Some(mut r) => {
                let _ = r.remove("password_hash");
                Ok(from_document(r)?)
            }
        }
    }

    pub async fn get_users(ctx: &Context, user_ids: Vec<ObjectId>) -> FieldResult<User> {
        let collection = ctx.db.collection(MONGO_DB_COLLECTION_NAME);
        let filter = doc! { "_id": {
                "$in": bson::to_bson(&user_ids)?,
            }
        };
        unimplemented!();
        match collection.find_one(filter, None).await? {
            None => {
                return Err(FieldError::new(
                    "specified user not found",
                    graphql_value!({ "error": "specified user not found" }),
                ))
            }
            Some(mut r) => {
                let _ = r.remove("password_hash");
                Ok(from_document::<User>(r)?)
            }
        }
    }
}

pub struct UserMutation;

impl UserMutation {
    pub async fn new_user(ctx: &Context, new_user: NewUser) -> FieldResult<User> {
        let user = User::new(new_user);
        let collection = ctx.db.collection(MONGO_DB_COLLECTION_NAME);
        let doc = to_document(&user)?;
        let _ = collection.insert_one(doc.clone(), None).await?;
        Ok(user)
    }

    pub async fn update_user(
        ctx: &Context,
        user_id: ObjectId,
        user_update: UserUpdate,
    ) -> FieldResult<User> {
        let collection = ctx.db.collection(MONGO_DB_COLLECTION_NAME);
        let mut user = UserQuery::get_user(ctx, user_id.clone()).await?;
        user.update(user_update);
        let filter = doc! { "_id": user_id };
        let user_doc = to_document(&user)?;
        let _ = collection.update_one(filter, user_doc, None).await?;
        Ok(user)
    }
}
