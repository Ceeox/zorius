use chrono::{DateTime, Utc};
use pwhash::sha512_crypt;
use sqlx::{
    postgres::{PgArgumentBuffer, PgValueRef},
    query_file, query_file_as, Decode, Encode, FromRow, PgPool, Postgres, Type,
};
use uuid::Uuid;

use crate::view::users::{NewUser, UserUpdate};

pub type UserId = Uuid;
pub type UserEmail = String;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub invitation_pending: bool,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
}

impl User {
    pub fn new(new_user: NewUser) -> Self {
        let password_hash = Self::hash_password(&new_user.password);
        Self {
            id: UserId::new_v4(),
            email: new_user.email,
            password_hash,
            firstname: Some(new_user.firstname),
            lastname: Some(new_user.lastname),
            created_at: Utc::now().into(),
            invitation_pending: true,
            updated_at: Utc::now().into(),
            deleted: false,
        }
    }

    pub fn get_password_hash(&self) -> &str {
        self.password_hash.as_ref()
    }

    pub fn change_password(&mut self, new_password: &str) {
        self.password_hash = Self::hash_password(new_password);
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn hash_password(password: &str) -> String {
        sha512_crypt::hash(password.as_bytes())
            .expect("system random number generator cannot be opened!")
    }

    pub fn is_password_correct(&self, password: &str) -> bool {
        sha512_crypt::verify(password.as_bytes(), &self.password_hash)
    }

    pub async fn get_dbuser_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<User, sqlx::Error> {
        Ok(query_file_as!(User, "sql/get_user_by_id.sql", id)
            .fetch_one(pool)
            .await?)
    }

    pub async fn get_dbuser_by_email(pool: &PgPool, email: String) -> Result<User, sqlx::Error> {
        Ok(query_file_as!(User, "sql/get_user_by_email.sql", email)
            .fetch_one(pool)
            .await?)
    }

    pub async fn list_users(
        pool: &PgPool,
        start: i64,
        limit: i64,
    ) -> Result<Vec<User>, sqlx::Error> {
        Ok(query_file_as!(User, "sql/list_users.sql", limit, start)
            .fetch_all(pool)
            .await?)
    }

    pub async fn count_users(pool: &PgPool) -> Result<i64, sqlx::Error> {
        Ok(query_file!("sql/count_users.sql")
            .fetch_one(pool)
            .await?
            .count
            .unwrap_or(0))
    }

    pub async fn new_user(pool: &PgPool, new_user: NewUser) -> Result<User, sqlx::Error> {
        let user = User::new(new_user);
        let new_user: User = query_file_as!(
            User,
            "sql/new_user.sql",
            user.id,
            user.email,
            user.password_hash,
            user.created_at,
            user.invitation_pending,
            user.firstname,
            user.lastname,
            user.updated_at,
            user.deleted
        )
        .fetch_one(pool)
        .await?;
        Ok(new_user.into())
    }

    // pub async fn new_user(pool: &PgPool, new_user: NewUser) -> Result<User, sqlx::Error> {
    //     let user = User::new(new_user);
    //     let new_user: User = query_file_as!(User, "sql/new_user2.sql", user as User)
    //         .fetch_one(pool)
    //         .await?;
    //     Ok(new_user.into())
    // }

    pub async fn update_user(
        pool: &PgPool,
        user_id: UserId,
        user_update: UserUpdate,
    ) -> Result<User, sqlx::Error> {
        Ok(query_file_as!(
            User,
            "sql/update_user.sql",
            user_id,
            user_update.firstname,
            user_update.lastname,
        )
        .fetch_one(pool)
        .await?
        .into())
    }

    pub async fn reset_password(
        pool: &PgPool,
        user_id: uuid::Uuid,
        password_hash: &str,
    ) -> Result<(), sqlx::Error> {
        let _ = query_file_as!(DBUser, "sql/reset_password.sql", password_hash, user_id)
            .fetch_one(pool)
            .await?;
        Ok(())
    }
}

impl<'r> Decode<'r, Postgres> for User {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        Ok(Self {
            id: decoder.try_decode::<UserId>()?,
            email: decoder.try_decode::<UserEmail>()?,
            password_hash: decoder.try_decode::<String>()?,
            firstname: decoder.try_decode::<Option<String>>()?,
            lastname: decoder.try_decode::<Option<String>>()?,
            created_at: decoder.try_decode::<DateTime<Utc>>()?,
            invitation_pending: decoder.try_decode::<bool>()?,
            updated_at: decoder.try_decode::<DateTime<Utc>>()?,
            deleted: decoder.try_decode::<bool>()?,
        })
    }
}

impl<'r> Encode<'r, Postgres> for User {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        let mut encoder = sqlx::postgres::types::PgRecordEncoder::new(buf);
        encoder.encode(&self.id);
        encoder.encode(&self.email);
        encoder.encode(&self.password_hash);
        encoder.encode(&self.created_at);
        encoder.encode(&self.invitation_pending);
        encoder.encode(&self.firstname);
        encoder.encode(&self.lastname);
        encoder.encode(&self.updated_at);
        encoder.encode(&self.deleted);
        encoder.finish();
        sqlx::encode::IsNull::No
    }
}
