use async_graphql::{
    validators::{Email, StringMaxLength, StringMinLength},
    InputObject, SimpleObject,
};
use chrono::{DateTime, Utc};
use pwhash::sha512_crypt;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::validators::Password;

pub type UserId = Uuid;
pub type UserEmail = String;

#[sqlx(type_name = "DBUser")]
#[derive(SimpleObject, Debug, Deserialize, Serialize, Clone, Type, FromRow)]
pub struct DBUser {
    pub id: UserId,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub invitation_pending: bool,
    pub firstname: String,
    pub lastname: String,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
}

impl DBUser {
    pub fn new(new_user: NewUser) -> Self {
        let password_hash = Self::hash_password(&new_user.password);
        Self {
            id: UserId::new_v4(),
            email: new_user.email,
            password_hash,
            firstname: new_user.firstname,
            lastname: new_user.lastname,
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
}

#[derive(SimpleObject, Debug, Deserialize, Serialize, Clone, FromRow, Type)]
pub struct User {
    pub id: UserId,
    pub email: UserEmail,
    pub created_at: DateTime<Utc>,
    pub firstname: String,
    pub lastname: String,
    pub updated_at: DateTime<Utc>,
}

impl From<DBUser> for User {
    fn from(db_user: DBUser) -> Self {
        Self {
            id: db_user.id,
            email: db_user.email,
            created_at: db_user.created_at,
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
    #[graphql(validator(Password))]
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(InputObject, Debug, Serialize)]
pub struct UserUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<UserEmail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
}
