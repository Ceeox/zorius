use std::collections::HashMap;

use async_graphql::{dataloader::Loader, Result};
use bson::{doc, from_document};
use mongodb::{Client, Database as MongoDB};

use crate::models::user::{User, UserId};

pub(crate) static MDB_COLL_NAME_USERS: &str = "users";
pub(crate) static MDB_COLL_WORK_ACCOUNTS: &str = "workaccounts";
pub(crate) static MDB_COLL_WORK_REPORTS: &str = "work_reports";
pub(crate) static MDB_COLL_INTERN_MERCH: &str = "merchandise_intern";
pub(crate) static MDB_COLL_ROLES: &str = "roles";

pub struct Database {
    client: Client,
    database: MongoDB,
}

impl Database {
    pub fn new(client: Client, database: mongodb::Database) -> Self {
        Self { client, database }
    }

    pub async fn get_user_by_id(&self, id: UserId) -> Result<Option<User>> {
        let collection = self.database.collection(MDB_COLL_NAME_USERS);
        let filter = doc! {"_id": id};
        let user = match collection.find_one(filter, None).await? {
            None => return Ok(None),
            Some(doc) => from_document::<User>(doc)?,
        };
        Ok(Some(user))
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<Option<User>> {
        let collection = self.database.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "email": email.clone() };
        let user = match collection.find_one(filter, None).await? {
            None => return Ok(None),
            Some(doc) => from_document::<User>(doc)?,
        };
        Ok(Some(user))
    }
}

#[async_trait::async_trait]
impl Loader<UserId> for Database {
    type Value = User;
    type Error = mongodb::error::Error;

    async fn load(&self, keys: &[UserId]) -> Result<HashMap<UserId, Self::Value>, Self::Error> {
        let mut hm = HashMap::new();

        let collection = self.database.collection(MDB_COLL_NAME_USERS);
        let filter = doc! {"_id": keys[0].clone()};
        let user = match collection.find_one(filter, None).await? {
            None => return Ok(hm),
            Some(doc) => from_document::<User>(doc)?,
        };
        hm.insert(keys[0].clone(), user);
        Ok(hm)
    }
}

#[async_trait::async_trait]
impl Loader<UserEmail> for Database {
    type Value = User;
    type Error = mongodb::error::Error;

    async fn load(
        &self,
        keys: &[UserEmail],
    ) -> Result<HashMap<UserEmail, Self::Value>, Self::Error> {
        let mut hm = HashMap::new();

        let collection = self.database.collection(MDB_COLL_NAME_USERS);
        let filter = doc! {"email": keys[0].clone()};
        let user = match collection.find_one(filter, None).await? {
            None => return Ok(hm),
            Some(doc) => from_document::<User>(doc)?,
        };
        hm.insert(keys[0].clone(), user);
        Ok(hm)
    }
}
