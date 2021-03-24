use std::convert::TryFrom;

use async_graphql::{
    validators::{Email, StringMaxLength, StringMinLength},
    InputObject, Result, SimpleObject,
};
use bson::{oid::ObjectId, to_document, Bson, DateTime, Document};
use chrono::Utc;
use mongod::{AsFilter, Collection, Comparator, Filter, Update};
use pwhash::sha512_crypt;
use serde::{Deserialize, Serialize};

use crate::helper::validators::Password;

pub type UserId = ObjectId;

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
}

impl Collection for User {
    const COLLECTION: &'static str = "users";

    fn from_document(document: Document) -> Result<Self, mongod::Error> {
        match bson::from_document::<Self>(document) {
            Ok(user) => Ok(user),
            Err(_) => Err(mongod::Error::invalid_document("missing required fields")),
        }
    }

    fn into_document(self) -> Result<Document, mongod::Error> {
        match bson::to_document::<Self>(&self) {
            Ok(doc) => Ok(doc),
            Err(_) => Err(mongod::Error::invalid_document("missing required fields")),
        }
    }
}

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

#[derive(Default)]
pub struct UserFilter {
    pub id: Option<Comparator<ObjectId>>,
    pub ids: Option<Comparator<ObjectId>>,
    pub email: Option<Comparator<String>>,
    pub firstname: Option<Comparator<String>>,
    pub lastname: Option<Comparator<String>>,
    pub username: Option<Comparator<String>>,
}

impl Filter for UserFilter {
    fn new() -> Self {
        Self::default()
    }

    fn into_document(self) -> Result<Document, mongod::Error> {
        let mut doc = Document::new();
        if let Some(value) = self.id {
            doc.insert("_id", mongod::ext::bson::Bson::try_from(value)?.0);
        }
        if let Some(value) = self.ids {
            doc.insert("_id", mongod::ext::bson::Bson::try_from(value)?.0);
        }
        if let Some(value) = self.email {
            doc.insert("email", mongod::ext::bson::Bson::try_from(value)?.0);
        }
        if let Some(value) = self.lastname {
            doc.insert("lastname", mongod::ext::bson::Bson::try_from(value)?.0);
        }
        if let Some(value) = self.firstname {
            doc.insert("firstname", mongod::ext::bson::Bson::try_from(value)?.0);
        }
        if let Some(value) = self.username {
            doc.insert("username", mongod::ext::bson::Bson::try_from(value)?.0);
        }
        Ok(doc)
    }
}

impl AsFilter<UserFilter> for User {
    fn filter() -> UserFilter {
        UserFilter::default()
    }

    fn into_filter(self) -> UserFilter {
        UserFilter {
            id: Some(Comparator::Eq(self.id)),
            ids: None,
            email: Some(Comparator::Eq(self.email)),
            username: Some(Comparator::Eq(self.username)),
            lastname: self.lastname.map_or(None, |v| Some(Comparator::Eq(v))),
            firstname: self.firstname.map_or(None, |v| Some(Comparator::Eq(v))),
        }
    }
}

#[derive(Default, InputObject)]
pub struct UsersFilter {
    pub id: Option<ObjectId>,
    pub ids: Option<Vec<ObjectId>>,
    pub email: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: Option<String>,
}

impl AsFilter<UserFilter> for UsersFilter {
    fn filter() -> UserFilter {
        UserFilter::default()
    }

    fn into_filter(self) -> UserFilter {
        UserFilter {
            id: self.id.map_or(None, |v| Some(Comparator::Eq(v))),
            ids: self.ids.map_or(None, |v| Some(Comparator::In(v))),
            email: self.email.map_or(None, |v| Some(Comparator::Eq(v))),
            username: self.username.map_or(None, |v| Some(Comparator::Eq(v))),
            lastname: self.lastname.map_or(None, |v| Some(Comparator::Eq(v))),
            firstname: self.firstname.map_or(None, |v| Some(Comparator::Eq(v))),
        }
    }
}

#[derive(Default, InputObject)]
pub struct SingleUserFilter {
    pub id: Option<UserId>,
    pub email: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: Option<String>,
}

impl AsFilter<UserFilter> for SingleUserFilter {
    fn filter() -> UserFilter {
        UserFilter::default()
    }

    fn into_filter(self) -> UserFilter {
        UserFilter {
            id: self.id.map_or(None, |v| Some(Comparator::Eq(v))),
            ids: None,
            email: self.email.map_or(None, |v| Some(Comparator::Eq(v))),
            username: self.username.map_or(None, |v| Some(Comparator::Eq(v))),
            lastname: self.lastname.map_or(None, |v| Some(Comparator::Eq(v))),
            firstname: self.firstname.map_or(None, |v| Some(Comparator::Eq(v))),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, InputObject, Default)]
pub struct UserUpdate {
    email: Option<String>,
    username: Option<String>,
    firstname: Option<String>,
    lastname: Option<String>,
}

impl Update for UserUpdate {
    fn new() -> Self {
        UserUpdate::default()
    }
    fn into_document(self) -> Result<Document, mongod::Error> {
        let mut doc = Document::new();
        if let Some(value) = self.email {
            doc.insert("email", value);
        }
        if let Some(value) = self.username {
            doc.insert("username", value);
        }
        if let Some(value) = self.firstname {
            doc.insert("firstname", value);
        }
        if let Some(value) = self.lastname {
            doc.insert("lastname", value);
        }
        Ok(doc)
    }
}
