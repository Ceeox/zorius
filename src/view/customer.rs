use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{
    models::{
        customer::{Customer as DbCustomer, CustomerId},
        project::ProjectId,
    },
    view::project::Project as ProjectView,
};

#[derive(Serialize, Debug, Clone, SimpleObject)]
pub struct Customer {
    pub id: CustomerId,
    pub name: String,
    pub identifier: String,
    pub note: Option<String>,
    pub projects: Option<Vec<ProjectView>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<DbCustomer> for Customer {
    fn from(db: DbCustomer) -> Self {
        Self {
            id: db.id,
            name: db.name,
            identifier: db.identifier,
            note: db.note,
            projects: None,
            created_at: db.created_at,
            updated_at: db.updated_at,
        }
    }
}

#[derive(Serialize, Debug, Clone, InputObject)]
pub struct NewCustomer {
    pub name: String,
    pub identifier: String,
    pub note: Option<String>,
    pub project_ids: Vec<ProjectId>,
}

#[derive(Serialize, InputObject)]
pub struct UpdateCustomer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Option<String>>,
}
