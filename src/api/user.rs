use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    validators::Email,
    Context, Error, Object, Result, Upload,
};
use chrono::{Duration, Utc};
use futures::stream::{self, StreamExt};

use crate::{
    api::{calc_list_params, claim::Claim, database},
    config::CONFIG,
    models::{
        auth::LoginResult,
        upload::{FileInfo, Storage},
        users::{
            count_users, list_users, new_user, reset_password, update_user, user_by_email,
            user_by_id, UserEmail, UserId,
        },
    },
    validators::Password,
    view::users::{NewUser, OrderBy, OrderDirection, User, UserUpdate},
};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn login(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(Email))] email: String,
        #[graphql(validator(Password))] password: String,
    ) -> Result<LoginResult> {
        let err = Error::new("email or password wrong!");
        let db = database(&ctx)?.db();

        let user = match user_by_email(db, &email).await? {
            None => return Err(Error::new("user not found")),
            Some(user) => user,
        };

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

    async fn get_user_by_id(&self, ctx: &Context<'_>, id: UserId) -> Result<Option<User>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(&ctx)?.db();
        if let Some(user) = user_by_id(db, id).await? {
            return Ok(Some(User::from(user)));
        }
        Ok(None)
    }

    async fn get_user_by_email(&self, ctx: &Context<'_>, email: UserEmail) -> Result<Option<User>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(&ctx)?.db();
        if let Some(user) = user_by_email(db, &email).await? {
            return Ok(Some(User::from(user)));
        }
        Ok(None)
    }

    //#[graphql(guard(RoleGuard(role = "Role::Admin")))]
    async fn list_users(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        order_by: Option<OrderBy>,
        dic: Option<OrderDirection>,
    ) -> Result<Connection<usize, User, EmptyFields, EmptyFields>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(&ctx)?.db();
        let count = count_users(db).await? as usize;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let (start, end, limit) = calc_list_params(count, after, before, first, last);

                let users = list_users(
                    db,
                    order_by.unwrap_or(OrderBy::CreatedAt),
                    dic.unwrap_or(OrderDirection::Asc),
                )
                .await?;

                let mut connection = Connection::new(start > 0, end < count);
                connection
                    .append_stream(
                        stream::iter(users)
                            .enumerate()
                            .map(|(n, db_user)| Edge::new(n + start, User::from(db_user))),
                    )
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
    async fn register(&self, ctx: &Context<'_>, new: NewUser) -> Result<Option<User>> {
        let db = database(&ctx)?.db();

        if !CONFIG.registration_enabled {
            return Err(Error::new("registration is not enabled"));
        }

        if user_by_email(db, &new.email).await?.is_some() {
            return Err(Error::new("email already registerd"));
        }

        let new_user = new_user(db, new).await?;
        if let Some(user) = new_user {
            return Ok(Some(User::from(user)));
        }
        Ok(None)
    }

    async fn reset_password(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(Password))] old_password: String,
        #[graphql(validator(Password))] new_password: String,
    ) -> Result<bool> {
        let auth_info = Claim::from_ctx(ctx)?;
        let user_id = auth_info.user_id();
        let db = database(&ctx)?.db();

        let user = match user_by_id(db, user_id.clone()).await? {
            None => return Ok(false),
            Some(user) => user,
        };

        if !user.is_password_correct(&old_password) {
            return Err(Error::new("old password is incorrect".to_owned()));
        } else {
            reset_password(db, user_id, &new_password).await?;
        }

        Ok(true)
    }

    //#[graphql(guard(RoleGuard(role = "Role::Admin")))]
    async fn update_user(
        &self,
        ctx: &Context<'_>,
        user_id: UserId,
        user_update: UserUpdate,
    ) -> Result<Option<User>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(&ctx)?.db();
        let updated_user = update_user(db, user_id, user_update).await?;
        if let Some(user) = updated_user {
            return Ok(Some(User::from(user)));
        }
        Ok(None)
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
