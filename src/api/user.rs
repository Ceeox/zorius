use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    guard::Guard,
    validators::{Email, StringMaxLength, StringMinLength},
    Context, Error, Object, Result, Upload,
};
use bson::from_document;
use chrono::{Duration, Utc};
use futures::StreamExt;

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

use super::{claim::Claim, database2};

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
        let user = database2(ctx)?.get_user_by_email(email.clone()).await?;

        if !user.is_password_correct(&password) {
            return Err(err);
        }
        let claim = Claim::new(
            email,
            user.get_id().clone(),
            (Utc::now() + Duration::seconds(CONFIG.token_lifetime)).timestamp() as usize,
        );
        let token = claim.to_string();

        Ok(LoginResult {
            token,
            expires_at: claim.expires_at(),
            user_id: user.get_id().to_owned(),
        })
    }

    async fn get_user_by_id(&self, ctx: &Context<'_>, id: UserId) -> Result<User> {
        let _ = Claim::from_ctx(ctx)?;
        Ok(database2(ctx)?.get_user_by_id(id).await?)
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
        let doc_count = database2(ctx)?.count_users().await?;

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
                let limit = (end - start) as i64;

                let cursor = database2(ctx)?.list_users(start as i64, limit).await?;

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
        Ok(database2(ctx)?.new_user(new_user).await?)
    }

    async fn reset_password(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(StringMaxLength(length = "64")))] old_password: String,
        #[graphql(validator(Password))] new_password: String,
    ) -> Result<bool> {
        let auth_info = Claim::from_ctx(ctx)?;
        let user_id = auth_info.user_id();

        let mut user = database2(ctx)?.get_user_by_id(user_id.clone()).await?;

        if !user.is_password_correct(&old_password) {
            return Err(Error::new("old password is wrong!".to_owned()));
        } else {
            user.change_password(&new_password);
        }

        let _ = database2(ctx)?.reset_password(user.get_id().clone(), user.get_password_hash());

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
        Ok(database2(ctx)?.update_user(user_id, user_update).await?)
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
