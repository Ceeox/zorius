use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{customer, project::Model},
    view::customer::Customer,
};

#[derive(Serialize, Debug, Clone, SimpleObject)]
pub struct Project {
    pub id: Uuid,
    pub customer: Option<Customer>,
    pub name: String,
    pub note: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, Debug, Clone, InputObject)]
pub struct NewProject {
    pub name: String,
    pub customer_id: Uuid,
    pub note: Option<String>,
}

impl From<(Model, Option<customer::Model>)> for Project {
    fn from((project, customer): (Model, Option<customer::Model>)) -> Self {
        let customer = match customer {
            None => None,
            Some(c) => Some(Customer {
                id: c.id,
                name: c.name,
                identifier: c.identifier,
                note: c.note,
                projects: None,
                created_at: c.created_at,
                updated_at: c.updated_at,
            }),
        };
        Self {
            id: project.id,
            customer,
            name: project.name,
            note: project.note,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}
