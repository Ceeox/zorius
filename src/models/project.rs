use async_graphql::{InputObject, SimpleObject};
use bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::models::user::{User, UserId};

pub type ProjectId = ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct Project {
    #[serde(rename = "_id")]
    pub id: ProjectId,
    pub creator_id: UserId,
    pub name: String,
    pub description: Option<String>,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct ProjectResponse {
    #[serde(rename = "_id")]
    pub id: ProjectId,
    pub creator: User,
    pub name: String,
    pub description: Option<String>,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject)]
pub struct NewProject {
    pub name: String,
    pub description: Option<String>,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct ProjectUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl Project {
    pub fn new(creator_id: UserId, new: NewProject) -> Self {
        Self {
            id: ProjectId::new(),
            creator_id,
            name: new.name,
            description: new.description,
            note: new.note,
        }
    }

    pub fn get_id(&self) -> &ProjectId {
        &self.id
    }
}
