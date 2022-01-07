use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, FixedOffset, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    models::work_report::Model,
    view::{customer::Customer, project::Project, users::User},
};

#[derive(Serialize, Debug, InputObject)]
pub struct NewWorkReport {
    pub customer_id: Uuid,
    pub project_id: Option<Uuid>,
    pub description: String,
    pub invoiced: bool,
}

#[derive(Serialize, Debug, InputObject)]
pub struct WorkReportUpdate {
    pub customer: Option<Uuid>,
    pub project: Option<Uuid>,
    pub description: Option<String>,
    pub invoiced: Option<bool>,
    pub report_started: Option<DateTime<FixedOffset>>,
    pub report_ended: Option<DateTime<FixedOffset>>,
}

#[derive(SimpleObject, Debug, Serialize, Clone)]
pub struct WorkReport {
    pub id: Uuid,
    pub owner: Option<User>,
    pub customer: Option<Customer>,
    pub project: Option<Project>,
    pub description: String,
    pub invoiced: bool,
    pub report_started: DateTime<Utc>,
    pub report_ended: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// impl From<(Model, combined_work_report::Model)> for WorkReport {
//     fn from((model, combine): (Model, combined_work_report::Model)) -> Self {
//         Self {
//             id: model.id,
//             owner: combine.owner_id,
//             customer: model.customer,
//             project: model.project,
//             description: model.description,
//             invoiced: model.invoiced,
//             report_started: model.report_started,
//             report_ended: model.report_ended,
//             created_at: model.created_at,
//             updated_at: model.updated_at,
//         }
//     }
// }
