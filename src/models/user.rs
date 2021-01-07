use async_graphql::{
    validators::{Email, StringMaxLength, StringMinLength},
    InputObject, SimpleObject,
};
use bson::oid::ObjectId;
use bson::DateTime;
use chrono::Utc;
use pwhash::sha512_crypt;
use serde::{Deserialize, Serialize};

use crate::helper::validators::Password;

pub type UserId = ObjectId;

#[derive(Deserialize, Debug, InputObject)]
pub struct NewUser {
    #[graphql(validator(Email))]
    pub email: String,
    #[graphql(validator(and(StringMinLength(length = "4"), StringMaxLength(length = "64"))))]
    pub username: String,
    #[graphql(validator(Password))]
    pub password: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, SimpleObject)]
pub struct User {
    #[serde(rename = "_id")]
    id: UserId,
    email: String,
    #[graphql(skip)]
    password_hash: String,
    username: String,
    created_at: DateTime,
    #[graphql(visible = false)]
    invitation_pending: bool,
    avatar_url: Option<String>,
    firstname: Option<String>,
    lastname: Option<String>,
    last_updated: Option<DateTime>,
    #[graphql(visible = false)]
    deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Claim {
    pub sub: String,
    pub user_id: UserId,
    pub exp: usize,
}

impl User {
    pub fn new(new_user: NewUser) -> Self {
        let password_hash = User::hash_password(&new_user.password);
        Self {
            id: ObjectId::new(),
            email: new_user.email,
            password_hash,
            username: new_user.username,
            firstname: new_user.firstname,
            lastname: new_user.lastname,
            created_at: Utc::now().into(),
            invitation_pending: true,
            deleted: false,
            last_updated: Some(Utc::now().into()),
            avatar_url: None,
        }
    }

    pub fn change_password(&mut self, new_password: &str) {
        self.password_hash = User::hash_password(new_password);
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
    pub fn is_deleted(&self) -> bool {
        self.deleted
    }
}
