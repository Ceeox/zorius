use bson::doc;
use chrono::Utc;
use juniper::FieldResult;
use mongodb::bson::from_document;
use mongodb::bson::Bson;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::user::User;
use crate::Context;

static MONGO_DB_COLLECTION_NAME: &str = "users";

#[derive(Deserialize, Serialize, Debug, juniper::GraphQLInputObject)]
#[graphql(description = "new user data, used to insert to database")]
pub struct EmptyUser {
    test: String,
}

pub struct UserQuery;

impl UserQuery {
    pub async fn get_user(ctx: &Context, user_id: String) -> FieldResult<Option<User>> {
        let collection = ctx.db.collection(MONGO_DB_COLLECTION_NAME);
        let filter = doc! { "_id": user_id };
        match collection.find_one(filter, None).await? {
            None => return Ok(None),
            Some(mut r) => {
                let _ = r.remove("password_hash");
                Ok(Some(from_document::<User>(r)?))
            }
        }
    }

    pub async fn get_users(ctx: &Context, user_ids: Vec<Uuid>) -> FieldResult<Option<User>> {
        let collection = ctx.db.collection(MONGO_DB_COLLECTION_NAME);
        let filter = doc! { "_id": {
                "$in": bson::to_bson(&user_ids)?,
            }
        };

        match collection.find_one(filter, None).await? {
            None => return Ok(None),
            Some(mut r) => {
                let _ = r.remove("password_hash");
                Ok(Some(from_document::<User>(r)?))
            }
        }
    }
}

pub struct UserMutation;

impl UserMutation {
    pub fn change_password(ctx: &Context, password: String) -> FieldResult<()> {
        unimplemented!();
    }

    pub fn change_email(ctx: &Context, email: String) -> FieldResult<()> {
        unimplemented!();
    }

    pub async fn update_user(ctx: &Context, user: User) -> FieldResult<Option<User>> {
        let collection = ctx.db.collection(MONGO_DB_COLLECTION_NAME);
        let filter = doc! { "_id": user.id };
        let update = doc! {
            "$set": {
                "last_updated": Utc::now().timestamp(),
                "firstname": user.firstname.map_or(Bson::Null, |r| Bson::String(r)),
                "lastname": user.lastname.map_or(Bson::Null, |r| Bson::String(r)),
            }
        };
        match collection.find_one_and_update(filter, update, None).await? {
            None => return Ok(None),
            Some(r) => Ok(Some(from_document(r)?)),
        }
    }
}
