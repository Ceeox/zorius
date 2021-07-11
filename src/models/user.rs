use async_graphql::{
    validators::{Email, StringMaxLength, StringMinLength},
    InputObject, SimpleObject,
};
use chrono::{DateTime, Utc};
use pwhash::sha512_crypt;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::validators::Password;

pub type UserId = Uuid;
pub type UserEmail = String;

#[derive(SimpleObject, Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct DBUser {
    id: UserId,
    email: String,
    password_hash: String,
    username: String,
    created_at: DateTime<Utc>,
    invitation_pending: bool,
    avatar_url: Option<String>,
    firstname: Option<String>,
    lastname: Option<String>,
    updated_at: DateTime<Utc>,
    deleted: bool,
}

impl DBUser {
    pub fn new(new_user: NewUser) -> Self {
        let password_hash = Self::hash_password(&new_user.password);
        Self {
            id: UserId::new_v4(),
            email: new_user.email,
            password_hash,
            username: new_user.username,
            firstname: new_user.firstname,
            lastname: new_user.lastname,
            created_at: Utc::now().into(),
            invitation_pending: true,
            updated_at: Utc::now().into(),
            avatar_url: None,
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
}

#[derive(SimpleObject, Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct User {
    pub id: UserId,
    pub email: UserEmail,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub avatar_url: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl From<DBUser> for User {
    fn from(db_user: DBUser) -> Self {
        Self {
            id: db_user.id,
            email: db_user.email,
            username: db_user.username,
            created_at: db_user.created_at,
            avatar_url: db_user.avatar_url,
            firstname: db_user.firstname,
            lastname: db_user.lastname,
            updated_at: db_user.updated_at,
        }
    }
}

#[derive(Deserialize, Debug, InputObject)]
pub struct NewUser {
    #[graphql(validator(Email))]
    pub email: UserEmail,
    #[graphql(validator(and(StringMinLength(length = "4"), StringMaxLength(length = "64"))))]
    pub username: String,
    #[graphql(validator(Password))]
    pub password: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

#[derive(InputObject, Debug, Serialize)]
pub struct UserUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<UserEmail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}
