use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    guard::Guard,
    validators::{Email, StringMaxLength, StringMinLength},
    Context, Error, Object, Result, Upload,
};
use bson::{doc, from_document, to_document};
use chrono::{Duration, Utc};
use futures::StreamExt;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument};

use crate::{
    config::CONFIG,
    helper::validators::Password,
    models::{
        auth::LoginResult,
        roles::{Role, RoleGuard},
        upload::{FileInfo, Storage},
        user::{NewUser, User, UserId, UserUpdate},
    },
};

use super::{claim::Claim, database, database2, MDB_COLL_NAME_USERS};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn login(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(Email))] email: String,
        #[graphql(validator(and(StringMinLength(length = "8"), StringMaxLength(length = "64"))))]
        password: String,
    ) -> Result<LoginResult> {
        let err = Error::new("email or password wrong!");
        let user = match database2(ctx)?.get_user_by_email(email.clone()).await? {
            Some(r) => r,
            None => return Err(Error::new("user could not be found")),
        };

        if !user.is_password_correct(&password) {
            return Err(err);
        }
        let claim = Claim::new(
            email,
            user.get_id().clone(),
            (Utc::now() + Duration::seconds(CONFIG.token_lifetime)).timestamp() as usize,
        );
        let key = &EncodingKey::from_secret(&CONFIG.secret_key.as_bytes());
        let token = jsonwebtoken::encode(&Header::new(Algorithm::HS512), &claim, key)?;

        Ok(LoginResult {
            token: token,
            expires_at: claim.expires_at(),
            user_id: user.get_id().to_owned(),
        })
    }

    async fn get_user_by_id(&self, ctx: &Context<'_>, id: UserId) -> Result<Option<User>> {
        let _ = Claim::from_ctx(ctx)?;
        match database2(ctx)?.get_user_by_id(id).await? {
            Some(r) => Ok(Some(r)),
            None => Err(Error::new("user could not be found")),
        }
    }

    #[graphql(guard(RoleGuard(role = "Role::Admin")))]
    async fn list_users(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, User, EmptyFields, EmptyFields>> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let doc_count = collection.estimated_document_count(None).await? as usize;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = after.map(|after| after + 1).unwrap_or(0);
                let mut end = before.unwrap_or(doc_count);

                if let Some(first) = first {
                    end = (start + first).min(end);
                }
                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last };
                }
                let options = FindOptions::builder()
                    .skip(start as i64)
                    .limit(end as i64)
                    .build();
                let cursor = collection.find(None, options).await?;

                let mut connection = Connection::new(start > 0, end < doc_count);
                connection
                    .append_stream(cursor.enumerate().map(|(n, doc)| {
                        let merch = from_document::<User>(doc.unwrap()).unwrap();
                        Edge::with_additional_fields(n + start, merch, EmptyFields)
                    }))
                    .await;
                Ok(connection)
            },
        )
        .await
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn register(&self, ctx: &Context<'_>, new_user: NewUser) -> Result<User> {
        if !CONFIG.registration_enabled {
            return Err(Error::new("registration is not enabled"));
        }

        let user = User::new(new_user);
        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let doc = to_document(&user)?;
        let _ = collection.insert_one(doc.clone(), None).await?;
        Ok(user)
    }

    async fn reset_password(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(StringMaxLength(length = "64")))] old_password: String,
        #[graphql(validator(Password))] new_password: String,
    ) -> Result<bool> {
        let auth_info = Claim::from_ctx(ctx)?;
        let user_id = auth_info.user_id();

        let mut user = match database2(ctx)?.get_user_by_id(user_id.clone()).await? {
            Some(r) => r,
            None => return Err(Error::new("user could not be found")),
        };

        if !user.is_password_correct(&old_password) {
            return Err(Error::new("old password is wrong!".to_owned()));
        } else {
            user.change_password(&new_password);
        }

        let update = to_document(&user)?;
        let filter = doc! {"_id": user_id};
        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let _ = collection.update_one(filter, update, None).await?;

        Ok(true)
    }

    #[graphql(guard(RoleGuard(role = "Role::Admin")))]
    async fn update_user(
        &self,
        ctx: &Context<'_>,
        user_id: UserId,
        user_update: UserUpdate,
    ) -> Result<User> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "_id": user_id };

        let mut update = bson::Document::new();
        update.insert("$set", bson::to_bson(&user_update)?);

        let options = FindOneAndUpdateOptions::builder()
            .return_document(Some(ReturnDocument::After))
            .build();

        let user = match collection
            .find_one_and_update(filter, update, Some(options))
            .await?
        {
            None => return Err(Error::new("specified user not found")),
            Some(r) => r,
        };
        Ok(from_document(user)?)
    }

    async fn upload_avatar(&self, ctx: &Context<'_>, files: Vec<Upload>) -> Result<Vec<FileInfo>> {
        let _ = Claim::from_ctx(ctx)?;

        let mut infos = Vec::new();
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        for file in files {
            let entry = storage.vacant_entry();
            let upload = file.value(ctx).unwrap();
            let info = FileInfo {
                id: entry.key().into(),
                filename: upload.filename.clone(),
                mimetype: upload.content_type.clone(),
            };
            entry.insert(info.clone());
            infos.push(info)
        }
        Ok(infos)
    }
}
