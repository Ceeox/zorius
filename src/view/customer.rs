use async_graphql::{InputObject, SimpleObject};

use entity::{customer::Model, project};
use sea_orm::prelude::DateTimeUtc;
use serde::Serialize;
use uuid::Uuid;

use crate::view::project::Project;

#[derive(Serialize, Debug, Clone, SimpleObject)]
pub struct Customer {
    pub id: Uuid,
    pub name: String,
    pub identifier: String,
    pub note: Option<String>,
    pub projects: Option<Vec<Project>>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,
}

impl From<Model> for Customer {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            identifier: model.identifier,
            note: model.note,
            projects: None,
            created_at: model.created_at,
            updated_at: model.updated_at,
            deleted_at: model.deleted_at,
        }
    }
}

impl From<(Model, Vec<project::Model>)> for Customer {
    fn from((customer, projects): (Model, Vec<project::Model>)) -> Self {
        Self {
            id: customer.id,
            name: customer.name,
            identifier: customer.identifier,
            note: customer.note,
            projects: Some(
                projects
                    .into_iter()
                    .map(|project| Project {
                        id: project.id,
                        customer: None,
                        name: project.name,
                        note: project.note,
                        created_at: project.created_at,
                        updated_at: project.updated_at,
                        deleted_at: project.deleted_at,
                    })
                    .collect(),
            ),
            created_at: customer.created_at,
            updated_at: customer.updated_at,
            deleted_at: customer.deleted_at,
        }
    }
}

#[derive(Serialize, Debug, Clone, InputObject)]
pub struct NewCustomer {
    pub name: String,
    pub identifier: String,
    pub note: Option<String>,
    pub project_ids: Option<Vec<Uuid>>,
}

#[derive(Serialize, InputObject)]
pub struct UpdateCustomer {
    pub name: Option<String>,
    pub identifier: Option<String>,
    pub note: Option<Option<String>>,
}
