use async_graphql::validators::{Email, StringMaxLength, StringMinLength};
use async_graphql::{Context, Error, Object, Result};
use bson::{doc, from_document, oid::ObjectId};
use chrono::{Duration, Utc};
use futures::{future, StreamExt, TryStreamExt};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use mongodb::{options::FindOptions, Collection, Cursor};

use crate::{
    config::CONFIG,
    models::{
        auth::LoginResult,
        merchandise::intern_merchandise::InternMerchandise,
        user::{Claim, User, UserId},
    },
    API_VERSION,
};

use super::{database, is_autherized, MDB_COLL_NAME_INTERN, MDB_COLL_NAME_USERS};
pub struct RootQuery;

#[Object]
impl RootQuery {
    async fn api_version(&self) -> &str {
        API_VERSION
    }

    pub async fn login(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(Email))] email: String,
        #[graphql(validator(and(StringMinLength(length = "0"), StringMaxLength(length = "64"))))]
        password: String,
    ) -> Result<LoginResult> {
        let err = Error::new("email or password wrong!");
        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "email": email.clone() };
        let user: User = match collection.find_one(filter, None).await? {
            None => {
                return Err(err);
            }
            Some(r) => from_document(r).unwrap(),
        };

        if !user.is_password_correct(&password) {
            return Err(err);
        }
        let claims = Claim {
            sub: email,
            exp: (Utc::now() + Duration::days(30)).timestamp() as usize,
        };
        let key = &EncodingKey::from_secret(&CONFIG.secret_key.as_bytes());
        let token = jsonwebtoken::encode(&Header::new(Algorithm::HS512), &claims, key)?;

        Ok(LoginResult { token: token })
    }

    async fn table_data(&self, ctx: &Context<'_>) -> Result<Vec<InternMerchandise>> {
        is_autherized(ctx)?;
        let collection: Collection = database(ctx)?.collection("merchandise_intern");
        let find_opt = Some(FindOptions::builder().limit(50).build());
        let cursor = collection.find(None, find_opt).await?;
        let res = cursor
            .filter_map(|doc| async move {
                match doc {
                    Err(_) => None,
                    Ok(r) => Some(from_document::<InternMerchandise>(r)),
                }
            })
            .try_collect::<Vec<InternMerchandise>>()
            .await?;

        Ok(res)
    }

    async fn get_order(&self, ctx: &Context<'_>, id: ObjectId) -> Result<InternMerchandise> {
        is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_NAME_INTERN);
        let filter = doc! { "_id": id };
        match collection.find_one(Some(filter), None).await? {
            None => return Err(Error::new("intern order not found")),
            Some(r) => Ok(from_document(r)?),
        }
    }

    async fn get_user(&self, ctx: &Context<'_>, user_id: UserId) -> Result<User> {
        is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "_id": user_id };
        match collection.find_one(filter, None).await? {
            None => return Err(Error::new("specified user not found")),
            Some(r) => Ok(from_document(r)?),
        }
    }

    async fn get_users(&self, ctx: &Context<'_>, user_ids: Vec<UserId>) -> Result<Vec<User>> {
        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "_id": {
                "$in": bson::to_bson(&user_ids)?,
            }
        };
        let cursor: Cursor = collection.find(filter, None).await?;
        let res = cursor
            .filter(|doc| future::ready(doc.is_ok()))
            .map(|doc| from_document::<User>(doc.unwrap()).unwrap())
            .filter(|user| future::ready(!user.is_deleted()))
            .collect::<Vec<_>>()
            .await;

        Ok(res)
    }

    async fn list_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let find_opt = Some(FindOptions::builder().limit(50).build());
        let cursor: Cursor = collection.find(None, find_opt).await?;
        let res = cursor
            .filter_map(|doc| async move {
                match doc {
                    Err(_) => None,
                    Ok(r) => Some(from_document::<User>(r)),
                }
            })
            .try_collect::<Vec<_>>()
            .await?;

        return Ok(res);
    }
}
