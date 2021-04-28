use async_graphql::{InputObject, SimpleObject};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::{user::UserId, work_report::project::Project};

use super::project::ProjectUpdate;

pub type CustomerId = ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct Customer {
    #[serde(rename = "_id")]
    id: CustomerId,
    creator: UserId,
    name: String,
    identifier: String,
    note: Option<String>,
    projects: Option<Vec<Project>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct CustomerAdd {
    pub creator: UserId,
    pub name: String,
    pub identifier: String,
    pub note: Option<String>,
    pub projects: Option<Vec<Project>>,
}

impl Into<Customer> for CustomerAdd {
    fn into(self) -> Customer {
        Customer {
            id: CustomerId::new(),
            creator: self.creator,
            name: self.name,
            identifier: self.identifier,
            note: self.note,
            projects: None,
        }
    }
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct CustomerUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    creator: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    projects: Option<Vec<ProjectUpdate>>,
}

impl Customer {
    pub fn new(add: CustomerAdd) -> Self {
        add.into()
    }
}

impl Update<Option<UserId>> for UserId {
    fn update(&mut self, update: Option<UserId>) {
        if update.is_some() {
            *self = update.unwrap();
        }
    }
}

impl Update<Option<String>> for String {
    fn update(&mut self, update: Option<String>) {
        if update.is_some() {
            *self = update.unwrap();
        }
    }
}

trait Update<T> {
    fn update(&mut self, update: T);
}
