use async_graphql::{InputObject, Result, SimpleObject};
use bson::{oid::ObjectId, to_document, Document};
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
    pub fn new(creator: UserId, name: String, identifier: String, note: Option<String>) -> Self {
        Self {
            id: ObjectId::new(),
            creator,
            name,
            identifier,
            note,
            projects: None,
        }
    }

    pub fn update(update: &CustomerUpdate) -> Result<Document> {
        Ok(to_document(update)?)
    }
}
