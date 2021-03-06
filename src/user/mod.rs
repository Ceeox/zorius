use std::{convert::TryInto, path::PathBuf};

use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Subscription, Upload, UploadValue,
};
use chrono::{Duration, Utc};
use entity::user::Entity;
use futures::stream::{self, StreamExt};
use futures_util::{AsyncReadExt, Stream};
use image::{imageops::FilterType, DynamicImage, ImageFormat};
use log::{debug, error, info};
use mime::Mime;
use sea_orm::EntityTrait;
use tokio::task::spawn_blocking;
use uuid::Uuid;

use crate::{
    api::{database, MutationType},
    claim::Claim,
    config::CONFIG,
    errors::{Error, Result},
    guards::TokenGuard,
    simple_broker::SimpleBroker,
    upload::FileInfo,
    user::db::save_user_avatar,
    validators::Password,
};

use self::{
    db::{
        count_users, list_users, new_user, reset_password, update_user, user_by_email, user_by_id,
    },
    model::{DbListOptions, ListUserOptions, LoginResult, NewUser, User, UserChanged, UserUpdate},
};

mod db;
pub mod model;

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
    async fn users(
        &self,
        ctx: &Context<'_>,
        options: Option<ListUserOptions>,
    ) -> async_graphql::Result<Connection<usize, User, EmptyFields, EmptyFields>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(ctx)?;
        let options = options.unwrap_or_default();
        let count = count_users(db).await? as usize;
        let mut db_options = DbListOptions {
            ids: options.ids,
            ..Default::default()
        };

        query(
            options.after,
            options.before,
            options.first,
            options.last,
            |after, before, first, last| async move {
                let mut start = after.map(|after| after + 1).unwrap_or(0);
                let mut end = before.unwrap_or(count);
                if let Some(first) = first {
                    end = (start + first).min(end);
                }
                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last };
                }
                db_options.start = start as u64;
                db_options.limit = end as u64;

                let users = list_users(db, db_options).await?;

                let mut connection = Connection::new(start > 0, end < count);
                connection
                    .append_stream(
                        stream::iter(users)
                            .enumerate()
                            .map(|(n, db_user)| Edge::new(n + start, User::from(db_user))),
                    )
                    .await;
                Ok::<_, Error>(connection)
            },
        )
        .await
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn register(&self, ctx: &Context<'_>, mut new: NewUser) -> Result<Option<User>> {
        let db = database(ctx)?;

        if !CONFIG.registration_enabled {
            return Err(Error::RegistrationNotEnabled);
        }

        if user_by_email(db, &new.email).await?.is_some() {
            return Err(Error::EmailAlreadyRegistred);
        }

        if let Ok(claim) = Claim::from_ctx(ctx) {
            let user_id = claim.user_id()?;
            let model = Entity::find_by_id(user_id).one(db).await?;
            if let Some(model) = model {
                if !model.is_admin {
                    new.is_admin = None;
                }
            }
        }

        if count_users(db).await? < 1 {
            new.is_admin = Some(true);
        }

        let new_user = new_user(db, new).await?;
        if let Some(user) = new_user {
            SimpleBroker::publish(UserChanged {
                mutation_type: MutationType::Created,
                id: user.id,
            });
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
            SimpleBroker::publish(UserChanged {
                mutation_type: MutationType::Updated,
                id: user.id,
            });
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
                    return Err(Error::Image(e));
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

            SimpleBroker::publish(UserChanged {
                mutation_type: MutationType::Updated,
                id: user.id,
            });
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
