use actix_web::HttpResponse;
use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    guard::Guard,
    validators::{Email, StringMaxLength, StringMinLength},
    Context, Error, Object, Result, Upload,
};
use chrono::{Duration, Utc};

use crate::{
    config::CONFIG,
    models::{
        auth::LoginResult,
        roles::{Role, RoleGuard},
        upload::{FileInfo, Storage},
        users::{User as DbUser, UserEmail, UserId},
    },
    validators::Password,
    view::users::{NewUser, User, UserUpdate},
};

use super::{claim::Claim, database};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn login(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(Email))] email: String,
        #[graphql(validator(and(
            StringMinLength(length = "8"),
            StringMaxLength(length = "255")
        )))]
        password: String,
    ) -> Result<LoginResult> {
        let err = Error::new("email or password wrong!");

        let user = DbUser::get_dbuser_by_email(database(&ctx)?.get_pool(), email.clone()).await?;

        if !user.is_password_correct(&password) {
            return Err(err);
        }
        let claim = Claim::new(
            email,
            user.get_id().clone().to_string(),
            (Utc::now() + Duration::seconds(CONFIG.token_lifetime)).timestamp() as usize,
        );
        let token = claim.to_string();

        Ok(LoginResult { token })
    }

    async fn get_user_by_id(&self, ctx: &Context<'_>, id: UserId) -> Result<User> {
        let _ = Claim::from_ctx(ctx)?;
        Ok(User::from(
            DbUser::get_dbuser_by_id(database(&ctx)?.get_pool(), id).await?,
        ))
    }

    async fn get_user_by_email(&self, ctx: &Context<'_>, email: UserEmail) -> Result<User> {
        let _ = Claim::from_ctx(ctx)?;
        Ok(User::from(
            DbUser::get_dbuser_by_email(database(&ctx)?.get_pool(), email).await?,
        ))
    }

    //#[graphql(guard(RoleGuard(role = "Role::Admin")))]
    async fn list_users(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, User, EmptyFields, EmptyFields>> {
        let _ = Claim::from_ctx(ctx)?;
        let count = DbUser::count_users(database(&ctx)?.get_pool()).await? as usize;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = after
                    .map(|after: usize| after.saturating_add(1))
                    .unwrap_or(0);
                let mut end = before.unwrap_or(count);

                if let Some(first) = first {
                    end = (start.saturating_add(first)).min(end);
                }
                if let Some(last) = last {
                    start = if last > end.saturating_sub(start) {
                        end
                    } else {
                        end.saturating_sub(last)
                    };
                }
                let limit = (end.saturating_sub(start)) as i64;

                let users =
                    DbUser::list_users(database(&ctx)?.get_pool(), start as i64, limit).await?;

                let mut connection = Connection::new(start > 0, end < count);
                connection.append(users.into_iter().enumerate().map(|(n, db_user)| {
                    Edge::with_additional_fields(n + start, User::from(db_user), EmptyFields)
                }));
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
        Ok(DbUser::new_user(database(&ctx)?.get_pool(), new_user)
            .await?
            .into())
    }

    async fn reset_password(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(StringMaxLength(length = "64")))] old_password: String,
        #[graphql(validator(Password))] new_password: String,
    ) -> Result<bool> {
        let auth_info = Claim::from_ctx(ctx)?;
        let user_id = auth_info.user_id();

        let mut user =
            DbUser::get_dbuser_by_id(database(&ctx)?.get_pool(), user_id.clone()).await?;

        if !user.is_password_correct(&old_password) {
            return Err(Error::new("old password is wrong!".to_owned()));
        } else {
            user.change_password(&new_password);
        }

        let _ = DbUser::reset_password(
            database(&ctx)?.get_pool(),
            user.get_id().clone(),
            user.get_password_hash(),
        );

        Ok(true)
    }

    //#[graphql(guard(RoleGuard(role = "Role::Admin")))]
    async fn update_user(
        &self,
        ctx: &Context<'_>,
        user_id: UserId,
        user_update: UserUpdate,
    ) -> Result<User> {
        let _ = Claim::from_ctx(ctx)?;
        Ok(
            DbUser::update_user(database(&ctx)?.get_pool(), user_id, user_update)
                .await?
                .into(),
        )
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
