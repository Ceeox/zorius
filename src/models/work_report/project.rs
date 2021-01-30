use async_graphql::{InputObject, Result, SimpleObject};
use bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use crate::models::user::UserId;

pub type ProjectId = ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct Project {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ProjectId>,
    pub creator: UserId,
    pub name: String,
    pub description: Option<String>,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct ProjectUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl Project {
    pub fn new(
        creator: UserId,
        name: String,
        description: Option<String>,
        note: Option<String>,
    ) -> Self {
        Self {
            id: Some(ObjectId::new()),
            creator,
            name,
            description,
            note,
        }
    }

    /// This creates a new projectt without an id.
    /// Can use for embeded usage.
    pub fn new_without_id(
        creator: UserId,
        name: String,
        description: Option<String>,
        note: Option<String>,
    ) -> Self {
        Self {
            id: None,
            creator,
            name,
            description,
            note,
        }
    }

    pub fn update(update: ProjectUpdate) -> Result<Document> {
        Ok(to_document(&update)?)
    }
}
