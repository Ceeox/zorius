use std::future;

use bson::{doc, oid::ObjectId, to_document, Bson, DateTime};
use chrono::Utc;
use futures::{stream::StreamExt, TryStreamExt};
use juniper::{graphql_value, FieldError, FieldResult};
use mongodb::{
    bson::from_document,
    options::{FindOneAndReplaceOptions, FindOptions, ReturnDocument},
};
use mongodb::{options::FindOneAndUpdateOptions, Cursor};

use crate::models::user::{NewUserQuery, UpdateUserQuery, User, UserId, UserResponse};
use crate::{helper::NullKeyRemover, Context};

pub static MDB_COLL_NAME_USERS: &str = "users";

static MAX_USER_QUERY: usize = 50;

pub enum UserSearchOptions {
    ById(ObjectId),
    ByEmail(String),
    ByUsername(String),
}

pub struct UserQuery;

impl UserQuery {
    pub async fn get_user(ctx: &Context, user_id: UserId) -> FieldResult<UserResponse> {
        let user = UserQuery::get_user_by_options(ctx, UserSearchOptions::ById(user_id)).await?;
        Ok(user.into())
    }

    async fn get_user_by_options(
        ctx: &Context,
        user_ident: UserSearchOptions,
    ) -> FieldResult<User> {
        let collection = ctx.db.collection(MDB_COLL_NAME_USERS);
        let filter = match user_ident {
            UserSearchOptions::ById(user_id) => doc! { "_id": user_id },
            UserSearchOptions::ByEmail(email) => doc! { "email": email },
            UserSearchOptions::ByUsername(username) => doc! { "username": username },
        };
        match collection.find_one(filter, None).await? {
            None => {
                return Err(FieldError::new(
                    "specified user not found",
                    graphql_value!({ "error": "specified user not found" }),
                ))
            }
            Some(r) => Ok(from_document(r)?),
        }
    }

    pub async fn get_users(ctx: &Context, user_ids: Vec<UserId>) -> FieldResult<Vec<UserResponse>> {
        let collection = ctx.db.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "_id": {
                "$in": bson::to_bson(&user_ids)?,
            }
        };
        let cursor: Cursor = collection.find(filter, None).await?;
        let res = cursor
            .filter(|doc| future::ready(doc.is_ok()))
            .map(|doc| from_document::<UserResponse>(doc.unwrap()).unwrap())
            .filter(|doc| future::ready(!doc.deleted))
            .collect::<Vec<_>>()
            .await;

        Ok(res)
    }

    pub async fn list_users(ctx: &Context) -> FieldResult<Vec<UserResponse>> {
        let collection = ctx.db.collection(MDB_COLL_NAME_USERS);
        let find_opt = Some(FindOptions::builder().limit(MAX_USER_QUERY as i64).build());
        let cursor: Cursor = collection.find(None, find_opt).await?;
        let res = cursor
            .filter_map(|doc| async move {
                match doc {
                    Err(_) => None,
                    Ok(r) => Some(from_document::<UserResponse>(r)),
                }
            })
            .try_collect::<Vec<_>>()
            .await?;

        return Ok(res);
    }
}

pub struct UserMutation;

impl UserMutation {
    pub async fn new_user(ctx: &Context, new_user: NewUserQuery) -> FieldResult<UserResponse> {
        let user = User::new(new_user);
        let collection = ctx.db.collection(MDB_COLL_NAME_USERS);
        let doc = to_document(&user)?;
        let _ = collection.insert_one(doc.clone(), None).await?;
        Ok(user.into())
    }

    pub async fn update_user(
        ctx: &Context,
        user_id: UserId,
        user_update: UpdateUserQuery,
    ) -> FieldResult<UserResponse> {
        let collection = ctx.db.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "_id": user_id };

        let mut replacement = to_document(&user_update)?.remove_null_keys();
        replacement.push(doc! { "$set": {"last_updated": Bson::DateTime(Utc::now())}});

        let options = FindOneAndUpdateOptions::builder()
            .return_document(Some(ReturnDocument::After))
            .build();

        let user: User = match collection
            .find_one_and_update(filter, replacement, Some(options))
            .await?
        {
            None => {
                return Err(FieldError::new(
                    "specified user not found",
                    graphql_value!({ "error": "specified user not found" }),
                ))
            }
            Some(r) => from_document(r)?,
        };
        Ok(user.into())
    }
}
