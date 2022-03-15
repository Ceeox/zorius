use async_graphql::{InputObject, SimpleObject};
use entity::{customer, project, user, work_report::*};
use sea_orm::prelude::DateTimeUtc;
use serde::Serialize;
use uuid::Uuid;

use crate::view::{customer::Customer, project::Project, time_record::TimeRecord, users::User};

#[derive(Serialize, Debug, InputObject)]
pub struct NewWorkReport {
    pub customer_id: Uuid,
    pub project_id: Option<Uuid>,
    pub description: String,
    pub invoiced: bool,
}

#[derive(Serialize, Debug, InputObject)]
pub struct WorkReportUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiced: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_started: Option<DateTimeUtc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_ended: Option<DateTimeUtc>,
}

#[derive(SimpleObject, Debug, Serialize, Clone)]
pub struct WorkReport {
    pub id: Uuid,
    pub owner: Option<User>,
    pub customer: Option<Customer>,
    pub project: Option<Project>,
    pub description: String,
    pub invoiced: bool,
    pub time_records: Option<Vec<TimeRecord>>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}
impl From<Model> for WorkReport {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            owner: None,
            customer: None,
            project: None,
            description: model.description,
            invoiced: model.invoiced,
            time_records: None,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<(Model, user::Model)> for WorkReport {
    fn from((model, user_model): (Model, user::Model)) -> Self {
        Self {
            id: model.id,
            owner: Some(User::from(user_model)),
            customer: None,
            project: None,
            description: model.description,
            invoiced: model.invoiced,
            time_records: None,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<(Model, project::Model)> for WorkReport {
    fn from((model, project_model): (Model, project::Model)) -> Self {
        Self {
            id: model.id,
            owner: None,
            customer: None,
            project: Some(Project::from(project_model)),
            description: model.description,
            invoiced: model.invoiced,
            time_records: None,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<(Model, customer::Model)> for WorkReport {
    fn from((model, customer_model): (Model, customer::Model)) -> Self {
        Self {
            id: model.id,
            owner: None,
            customer: Some(Customer::from(customer_model)),
            project: None,
            description: model.description,
            invoiced: model.invoiced,
            time_records: None,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
