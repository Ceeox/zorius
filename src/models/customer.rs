use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{
        project::{Project, ProjectId},
        users::UserId,
    },
    view::users::User,
};

pub type CustomerId = Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct DBCustomer {
    id: CustomerId,
    creator_id: UserId,
    name: String,
    identifier: String,
    note: Option<String>,
    project_ids: Vec<ProjectId>,
}

impl DBCustomer {
    pub fn new(new: NewCustomer, creator_id: UserId) -> Self {
        Self {
            id: CustomerId::new_v4(),
            creator_id,
            name: new.name,
            identifier: new.identifier,
            note: new.note,
            project_ids: new.project_ids,
        }
    }

    pub fn get_id(&self) -> &CustomerId {
        &self.id
    }
}

#[derive(Serialize, Debug, Clone, SimpleObject)]
pub struct Customer {
    id: CustomerId,
    creator: User,
    name: String,
    identifier: String,
    note: Option<String>,
    projects: Vec<Project>,
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
    creator: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}
