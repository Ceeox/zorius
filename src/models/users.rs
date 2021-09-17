use chrono::{DateTime, Utc};
use pwhash::sha512_crypt;
use sqlx::{
    postgres::{PgArgumentBuffer, PgValueRef},
    query, query_as,
    types::Decimal,
    Decode, Encode, FromRow, PgPool, Postgres,
};
use uuid::Uuid;

use crate::{
    models::intern_merchandise::{InternMerchandiseId, InternMerchandiseStatus},
    view::users::{NewUser, UserUpdate},
};

pub type UserId = Uuid;
pub type UserEmail = String;

#[derive(Debug, Clone, FromRow)]
pub struct UserEntity {
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

impl UserEntity {
    pub async fn new(pool: &PgPool, new_user: NewUser) -> Result<Self, sqlx::Error> {
        Ok(query_as!(
            UserEntity,
            r#"INSERT INTO users (
                email,
                password_hash,
                invitation_pending,
                firstname,
                lastname
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5
            )
            RETURNING *;"#,
            new_user.email,
            UserEntity::hash_password(&new_user.password),
            true,
            new_user.firstname,
            new_user.lastname
        )
        .fetch_one(pool)
        .await?)
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    fn hash_password(password: &str) -> String {
        sha512_crypt::hash(password.as_bytes())
            .expect("system random number generator cannot be opened!")
    }

    pub fn is_password_correct(&self, password: &str) -> bool {
        sha512_crypt::verify(password.as_bytes(), &self.password_hash)
    }

    pub async fn user_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<Self, sqlx::Error> {
        Ok(query_as!(
            UserEntity,
            r#"SELECT *
            FROM users
            WHERE id = $1"#,
            id
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn user_by_email(pool: &PgPool, email: &str) -> Result<Self, sqlx::Error> {
        Ok(query_as!(
            UserEntity,
            r#"SELECT *
            FROM users
            WHERE email = $1"#,
            email
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn list_users(
        pool: &PgPool,
        start: i64,
        limit: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(query_as!(
            UserEntity,
            r#"SELECT *
            FROM users
            ORDER BY created_at ASC
            LIMIT $1
            OFFSET $2;"#,
            limit,
            start
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn count_users(pool: &PgPool) -> Result<i64, sqlx::Error> {
        Ok(query!(
            r#"SELECT COUNT(id) 
            FROM users;"#
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0))
    }

    pub async fn update_user(
        pool: &PgPool,
        user_id: UserId,
        user_update: UserUpdate,
    ) -> Result<Self, sqlx::Error> {
        Ok(query_as!(
            UserEntity,
            r#"UPDATE users
                SET firstname = $2,
                lastname = $3
            WHERE id = $1
            RETURNING *;"#,
            user_id,
            user_update.firstname,
            user_update.lastname,
        )
        .fetch_one(pool)
        .await?
        .into())
    }

    pub async fn reset_password(
        &self,
        pool: &PgPool,
        password_hash: &str,
    ) -> Result<(), sqlx::Error> {
        let _ = query_as!(
            DBUser,
            r#"UPDATE users SET password_hash = $1 WHERE id = $2;"#,
            password_hash,
            self.id
        )
        .fetch_one(pool)
        .await?;
        Ok(())
    }
}

impl<'r> Decode<'r, Postgres> for UserEntity {
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

impl<'r> Encode<'r, Postgres> for UserEntity {
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
