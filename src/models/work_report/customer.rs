use async_graphql::{InputObject, SimpleObject};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::user::{User, UserId};

pub type CustomerId = ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct Customer {
    #[serde(rename = "_id")]
    id: CustomerId,
    creator: UserId,
    name: String,
    identifier: String,
    note: Option<String>,
}

impl Customer {
    pub fn new(new: NewCustomer, creator: UserId) -> Self {
        Self {
            id: CustomerId::new(),
            creator,
            name: new.name,
            identifier: new.identifier,
            note: new.note,
        }
    }
    pub fn get_id(&self) -> &CustomerId {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct CustomerResponse {
    #[serde(rename = "_id")]
    id: CustomerId,
    creator: User,
    name: String,
    identifier: String,
    note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject)]
pub struct NewCustomer {
    pub name: String,
    pub identifier: String,
    pub note: Option<String>,
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
}
