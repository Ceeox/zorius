use async_graphql::{
    guard::Guard,
    validators::{Email, StringMaxLength, StringMinLength},
    Context, Error, Object, Result, Upload,
};
use bson::{doc, from_document, to_document, Bson};
use chrono::{Duration, Utc};
use futures::{StreamExt, TryStreamExt};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use mongod::{AsFilter, Client};
use mongodb::{
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Cursor,
};

use crate::{
    config::CONFIG,
    helper::validators::Password,
    models::{
        auth::LoginResult,
        roles::{Role, RoleGuard},
        upload::{FileInfo, Storage},
        user::{NewUser, SingleUserFilter, User, UserId, UserUpdate},
    },
};

use super::{claim::Claim, database, MDB_COLL_NAME_USERS};

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
    /*
        async fn get_user(&self, ctx: &Context<'_>, user_id: UserId) -> Result<User> {
            let _ = Claim::from_ctx(ctx)?;
            let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
            let filter = doc! { "_id": user_id };
            match collection.find_one(filter, None).await? {
                None => return Err(Error::new("specified user not found")),
                Some(r) => Ok(from_document(r)?),
            }
        }
    */

    async fn get_user(&self, ctx: &Context<'_>, filter: SingleUserFilter) -> Result<Option<User>> {
        let _ = Claim::from_ctx(ctx)?;
        let client = ctx.data::<Client>()?;
        let filter = filter.into_filter();
        let user = client.find_one::<User, _>(filter).await?;
        Ok(user)
    }

    /*     async fn get_users(&self, ctx: &Context<'_>, user_ids: Vec<UserId>) -> Result<Vec<User>> {
        let _ = Claim::from_ctx(ctx)?;
        let client = ctx.data::<Client>()?;
        let mut filter = DbUser::filter();
        filter.ids = Some(Comparator::In(vec![user_ids]));
        let cursor: Cursor = client.find::<DbUser, _>(Some(filter)).await?;
        let users = cursor
            .filter_map(|docs| async move {
                match docs {
                    Ok(doc) => match from_document::<DbUser>(doc) {
                        Ok(db_user) => Some(User::from(db_user)),
                        Err(_) => None,
                    },
                    Err(_) => None,
                }
            })
            .collect::<Vec<_>>()
            .await;
        println!("{:#?}", users);
        Ok(users)
    } */

    async fn list_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let _ = Claim::from_ctx(ctx)?;
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

        Ok(res)
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
        Ok(user.into())
    }

    async fn reset_password(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(StringMaxLength(length = "64")))] old_password: String,
        #[graphql(validator(Password))] new_password: String,
    ) -> Result<bool> {
        let auth_info = Claim::from_ctx(ctx)?;
        let user_id = auth_info.user_id();

        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "_id": user_id };
        let mut user: User = match collection.find_one(filter.clone(), None).await? {
            None => return Err(Error::new("specified user not found".to_owned())),
            Some(r) => from_document(r)?,
        };

        if !user.is_password_correct(&old_password) {
            return Err(Error::new("old password is wrong!".to_owned()));
        } else {
            user.change_password(&new_password);
        }

        let update = to_document(&user)?;
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

        let mut update = User::update(&user_update)?;
        update.insert("last_updated", Bson::DateTime(Utc::now()));
        update = doc! { "$set" : update };
        println!("{:#?}", update);

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
