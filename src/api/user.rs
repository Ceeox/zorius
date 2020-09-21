use std::sync::Mutex;

use bson::{bson, doc, from_bson, to_bson, Bson};
use chrono::{DateTime, Utc};
use juniper::{FieldResult, GraphQLObject, RootNode};
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::ZoriusError;
use crate::models::user::{NewUser, User};
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
        let item = match collection.find_one(filter, None).await? {
            None => return Ok(None),
            Some(mut r) => {
                let _ = r.remove("password_hash");
                to_bson(&r)?
            }
        };
        let res = from_bson(item)?;
        println!("{:?}", res);
        Ok(Some(res))
    }

    pub async fn get_users(ctx: &Context, user_ids: Vec<Uuid>) -> FieldResult<Option<User>> {
        let collection = ctx.db.collection(MONGO_DB_COLLECTION_NAME);
        let filter = doc! { "_id": {
                "$in": bson::to_bson(&user_ids)?,
            }
        };
        let item = match collection.find_one(filter, None).await? {
            None => return Ok(None),
            Some(mut r) => {
                let _ = r.remove("password_hash");
                to_bson(&r)?
            }
        };
        let res = from_bson(item)?;
        println!("{:?}", res);
        Ok(Some(res))
    }
}

pub struct UserMutation;

impl UserMutation {
    pub fn change_password(ctx: &Context, password: String) -> FieldResult<()> {
        Ok(())
    }

    pub fn change_email(ctx: &Context, email: String) -> FieldResult<()> {
        Ok(())
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
        let item = match collection.find_one_and_update(filter, update, None).await? {
            None => return Ok(None),
            Some(r) => to_bson(&r)?,
        };
        let res = from_bson(item)?;
        println!("{:?}", res);
        Ok(Some(res))
    }
}
