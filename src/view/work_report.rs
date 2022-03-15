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

impl
    From<(
        Model,
        Option<user::Model>,
        Option<customer::Model>,
        Option<project::Model>,
    )> for WorkReport
{
    fn from(
        (model, owner_model, customer_model, project_model): (
            Model,
            Option<user::Model>,
            Option<customer::Model>,
            Option<project::Model>,
        ),
    ) -> Self {
        Self {
            id: model.id,
            owner: owner_model.map(User::from),
            customer: customer_model.map(Customer::from),
            project: project_model.map(Project::from),
            description: model.description,
            invoiced: model.invoiced,
            time_records: None,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
