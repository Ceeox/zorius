use async_graphql::SimpleObject;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::user::UserId;

pub type CustomerId = ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct Customer {
    id: CustomerId,
    creator: UserId,
    name: String,
    identifier: String,
}

impl Customer {
    pub fn new(creator: UserId, name: String, identifier: String) -> Self {
        Self {
            id: ObjectId::new(),
            creator,
            name,
            identifier,
        }
    }
}
