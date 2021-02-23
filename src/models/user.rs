use async_graphql::{
    validators::{Email, StringMaxLength, StringMinLength},
    InputObject, Result, SimpleObject,
};
use bson::{oid::ObjectId, to_document, DateTime, Document};
use chrono::Utc;
use mongod::Bson;
use mongod::Mongo;
use mongod::{AsFilter, AsUpdate, Collection, Comparator, Updates};
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

#[derive(Mongo, Debug, Deserialize, Serialize)]
#[mongo(bson = "serde", collection = "users", field, update)]
pub struct DbUser {
    #[serde(rename = "_id")]
    id: UserId,
    email: String,
    password_hash: String,
    username: String,
    created_at: DateTime,
    invitation_pending: bool,
    avatar_url: Option<String>,
    firstname: Option<String>,
    lastname: Option<String>,
    updated: DateTime,
    deleted: bool,
}

impl From<DbUser> for User {
    fn from(db_user: DbUser) -> User {
        User {
            id: db_user.id,
            email: db_user.email,
            username: db_user.username,
            created_at: db_user.created_at,
            invitation_pending: db_user.invitation_pending,
            avatar_url: db_user.avatar_url,
            firstname: db_user.firstname,
            lastname: db_user.lastname,
            updated: db_user.updated,
            deleted: db_user.deleted,
            password_hash: db_user.password_hash,
        }
    }
}

#[derive(Default)]
pub struct UserFilter {
    pub id: Option<Comparator<ObjectId>>,
    pub email: Option<Comparator<String>>,
    pub ids: Option<Comparator<Vec<UserId>>>,
}

impl mongod::Filter for UserFilter {
    fn new() -> Self {
        Self::default()
    }

    fn into_document(self) -> Result<Document, mongod::Error> {
        use std::convert::TryFrom;

        let mut doc = Document::new();
        if let Some(value) = self.email {
            doc.insert("email", mongod::ext::bson::Bson::try_from(value)?.0);
        }
        if let Some(value) = self.id {
            doc.insert("_id", mongod::ext::bson::Bson::try_from(value)?.0);
        }
        if let Some(value) = self.ids {
            doc.insert("_id", mongod::ext::bson::Bson::try_from(value)?.0);
        }
        Ok(doc)
    }
}

impl AsFilter<UserFilter> for DbUser {
    fn filter() -> UserFilter {
        UserFilter::default()
    }

    fn into_filter(self) -> UserFilter {
        UserFilter {
            email: Some(Comparator::Eq(self.email)),
            id: Some(Comparator::Eq(self.id)),
            ids: None,
        }
    }
}

#[derive(SimpleObject, Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")]
    id: UserId,
    email: String,
    #[graphql(skip)]
    password_hash: String,
    username: String,
    created_at: DateTime,
    #[graphql(skip)]
    invitation_pending: bool,
    avatar_url: Option<String>,
    firstname: Option<String>,
    lastname: Option<String>,
    updated: DateTime,
    #[graphql(skip)]
    deleted: bool,
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
            updated: Utc::now().into(),
            avatar_url: None,
        }
    }

    pub fn update(update: &UserUpdate) -> Result<Document> {
        Ok(to_document(update)?)
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

#[derive(Deserialize, Serialize, Debug, InputObject)]
pub struct UserUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lastname: Option<String>,
}
