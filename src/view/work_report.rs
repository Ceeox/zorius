use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::{customer::CustomerId, project::ProjectId};

#[derive(Serialize, Debug, InputObject)]
pub struct NewWorkReport {
    pub customer_id: CustomerId,
    pub project_id: Option<ProjectId>,
    pub description: String,
    pub invoiced: bool,
}

#[derive(Serialize, Debug, InputObject)]
pub struct WorkReportUpdate {
    customer: Option<CustomerId>,
    project: Option<ProjectId>,
    description: Option<String>,
    invoiced: Option<bool>,
    report_started: Option<DateTime<Utc>>,
    report_ended: Option<DateTime<Utc>>,
}
