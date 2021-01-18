use async_graphql::SimpleObject;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::user::UserId;

pub type ProjectId = ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct Project {
    pub id: ProjectId,
    pub creator: UserId,
    pub name: String,
}

impl Project {
    pub fn new(creator: UserId, name: String) -> Self {
        Self {
            id: ObjectId::new(),
            creator,
            name,
        }
    }
}
