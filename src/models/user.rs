use async_graphql::{
    validators::{Email, StringMaxLength, StringMinLength},
    InputObject, SimpleObject,
};
use bson::{oid::ObjectId, DateTime};
use chrono::Utc;
use pwhash::sha512_crypt;
use serde::{Deserialize, Serialize};

use crate::helper::validators::Password;

pub type UserId = ObjectId;
pub type UserEmail = String;

#[derive(SimpleObject, Debug, Deserialize, Serialize, Clone)]
pub struct DBUser {
    #[serde(rename = "_id")]
    id: UserId,
    pub email: UserEmail,
    password_hash: String,
    pub username: String,
    pub created_at: DateTime,
    pub invitation_pending: bool,
    pub avatar_url: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub updated: DateTime,
    pub deleted: bool,
}

impl DBUser {
    pub fn new(new_user: NewUser) -> Self {
        let password_hash = Self::hash_password(&new_user.password);
        Self {
            id: ObjectId::new(),
            email: new_user.email,
            password_hash,
            username: new_user.username,
            firstname: new_user.firstname,
            lastname: new_user.lastname,
            created_at: Utc::now().into(),
            invitation_pending: true,
            updated: Utc::now().into(),
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

    pub fn get_id(&self) -> &UserId {
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

#[derive(SimpleObject, Debug, Deserialize, Serialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    id: UserId,
    pub email: UserEmail,
    pub username: String,
    pub created_at: DateTime,
    pub avatar_url: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub updated: DateTime,
    pub deleted: bool,
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
    email: Option<UserEmail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<String>,
    firstname: Option<String>,
    lastname: Option<String>,
}
