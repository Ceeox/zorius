use std::{convert::TryInto, path::PathBuf};

use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Subscription, Upload, UploadValue,
};
use chrono::{Duration, Utc};
use futures::stream::{self, StreamExt};
use futures_util::{AsyncReadExt, Stream};
use image::{imageops::FilterType, DynamicImage, ImageFormat};
use log::{debug, error, info};
use mime::Mime;
use tokio::task::spawn_blocking;
use uuid::Uuid;

use crate::{
    api::{
        calc_list_params, claim::Claim, database, guards::TokenGuard, simple_broker::SimpleBroker,
        MutationType,
    },
    config::CONFIG,
    errors::{Error, Result},
    models::{
        auth::LoginResult,
        upload::FileInfo,
        users::{
            count_users, list_users, new_user, reset_password, save_user_avatar, update_user,
            user_by_email, user_by_id,
        },
    },
    validators::Password,
    view::users::{NewUser, OrderBy, OrderDirection, User, UserChanged, UserUpdate},
};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn login(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(email))] email: String,
        #[graphql(validator(custom = "Password"))] password: String,
    ) -> Result<LoginResult> {
        let db = database(ctx)?;

        let user = match user_by_email(db, &email).await? {
            None => return Err(Error::NotFound),
            Some(user) => user,
        };
        let user = User::from(user);

        if !user.is_password_correct(&password) {
            return Err(Error::IncorrectPassword);
        }
        let claim = Claim::new(
            &email,
            &user.get_id().clone().to_string(),
            (Utc::now() + Duration::seconds(CONFIG.token_lifetime)).timestamp() as usize,
        );
        let token = claim.try_into()?;

        Ok(LoginResult { token })
    }

    #[graphql(guard = "TokenGuard")]
    async fn get_user_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<User>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(ctx)?;
        if let Some(user) = user_by_id(db, id).await? {
            return Ok(Some(User::from(user)));
        }
        Ok(None)
    }

    #[graphql(guard = "TokenGuard")]
    async fn get_user_by_email(&self, ctx: &Context<'_>, email: String) -> Result<Option<User>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(ctx)?;
        if let Some(user) = user_by_email(db, &email).await? {
            return Ok(Some(User::from(user)));
        }
        Ok(None)
    }

    #[graphql(guard = "TokenGuard")]
    async fn list_users(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> async_graphql::Result<Connection<usize, User, EmptyFields, EmptyFields>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(ctx)?;
        let count = count_users(db).await? as usize;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let (start, end, limit) = calc_list_params(count, after, before, first, last);

                let users =
                    match list_users(db, OrderBy::CreatedAt, OrderDirection::Asc, start, limit)
                        .await
                    {
                        Ok(r) => r,
                        Err(_e) => return Err(async_graphql::Error::new("")),
                    };

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
        let db = database(ctx)?;

        if !CONFIG.registration_enabled {
            return Err(Error::RegistrationNotEnabled);
        }

        if user_by_email(db, &new.email).await?.is_some() {
            return Err(Error::EmailAlreadyRegistred);
        }

        let new_user = new_user(db, new).await?;
        if let Some(user) = new_user {
            return Ok(Some(User::from(user)));
        }
        Ok(None)
    }

    #[graphql(guard = "TokenGuard")]
    async fn reset_password(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(custom = "Password"))] old_password: String,
        #[graphql(validator(custom = "Password"))] new_password: String,
    ) -> Result<bool> {
        let auth_info = Claim::from_ctx(ctx)?;
        let user_id = auth_info.user_id()?;
        let db = database(ctx)?;

        let user = match user_by_id(db, user_id).await? {
            None => return Ok(false),
            Some(user) => user,
        };
        let user = User::from(user);

        if !user.is_password_correct(&old_password) {
            return Err(Error::IncorrectPassword);
        } else {
            reset_password(db, user_id, &new_password).await?;
        }

        Ok(true)
    }

    #[graphql(guard = "TokenGuard")]
    async fn update_user(
        &self,
        ctx: &Context<'_>,
        user_id: Uuid,
        user_update: UserUpdate,
    ) -> Result<Option<User>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(ctx)?;
        let updated_user = update_user(db, user_id, user_update).await?;
        if let Some(user) = updated_user {
            return Ok(Some(User::from(user)));
        }
        Ok(None)
    }

    #[graphql(guard = "TokenGuard")]
    async fn upload_avatar(&self, ctx: &Context<'_>, file: Upload) -> Result<FileInfo> {
        debug!("avatar upload");
        let claim = Claim::from_ctx(ctx)?;
        let db = database(ctx)?;
        let user_id = claim.user_id()?;

        let file_name = format!("{}.png", Uuid::new_v4());
        let path: PathBuf = ["static", "avatar", &file_name].iter().collect();

        let value: UploadValue = file.value(ctx).unwrap();
        let format: ImageFormat = match value.content_type {
            Some(ref content_type) => match content_type.parse::<Mime>() {
                Ok(r) if r == mime::IMAGE_PNG => ImageFormat::Png,
                Ok(r) if r == mime::IMAGE_JPEG => ImageFormat::Jpeg,
                _ => return Err(Error::WrongMediaType),
            },
            None => return Err(Error::WrongMediaType),
        };

        let mut vec = Vec::new();
        let mut reader = value.into_async_read();
        reader.read_to_end(&mut vec).await.unwrap();

        let path2 = PathBuf::clone(&path);
        match spawn_blocking(move || {
            debug!("loading image from memory");
            let img: DynamicImage = match image::load_from_memory_with_format(&vec, format) {
                Err(e) => {
                    error!("failed to load image from memory: {e}");
                    return Err(Error::ImageError(e));
                }
                Ok(r) => r,
            };
            debug!("starting to resize avatar");
            let img = img.resize(512, 512, FilterType::Lanczos3);
            debug!("resized image to 512x512");
            let _ = img.save_with_format(path2, ImageFormat::Png);
            debug!("saved image in fs");
            Ok(())
        })
        .await
        {
            Ok(_) => info!("saved new avatar"),
            Err(e) => {
                error!("failed to save avatar image: {e:?}");
                return Err(Error::Unknown);
            }
        }

        if let Some(user) = user_by_id(db, user_id).await? {
            if let Some(old_avatar) = user.avatar_filename {
                let old_avatar_path: PathBuf = ["static", "avatar", &old_avatar].iter().collect();
                let _ = tokio::fs::remove_file(old_avatar_path).await;
            }

            let _ = save_user_avatar(db, user_id, file_name.clone()).await?;
        }

        Ok(FileInfo {
            filename: file_name,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct UserSubscription;

#[Subscription]
impl UserSubscription {
    #[graphql(guard = "TokenGuard")]
    async fn users(&self, mutation_type: Option<MutationType>) -> impl Stream<Item = UserChanged> {
        SimpleBroker::<UserChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}
