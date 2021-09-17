use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{
    customer::CustomerId,
    project::{ProjectEntity, ProjectId},
};

#[derive(Serialize, Debug, Clone, SimpleObject)]
pub struct Project {
    pub id: ProjectId,
    pub customer_id: CustomerId,
    pub name: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone, InputObject)]
pub struct NewProject {
    pub name: String,
    pub customer_id: CustomerId,
    pub note: Option<String>,
}

impl From<ProjectEntity> for Project {
    fn from(db: ProjectEntity) -> Self {
        Self {
            id: db.id,
            customer_id: db.customer_id,
            name: db.name,
            note: db.note,
            created_at: db.created_at,
            updated_at: db.updated_at,
        }
    }
}
