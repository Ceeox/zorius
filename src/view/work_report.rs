use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, InputObject)]
pub struct NewWorkReport {
    pub customer_id: Uuid,
    pub project_id: Option<Uuid>,
    pub description: String,
    pub invoiced: bool,
}

#[derive(Serialize, Debug, InputObject)]
pub struct WorkReportUpdate {
    customer: Option<Uuid>,
    project: Option<Uuid>,
    description: Option<String>,
    invoiced: Option<bool>,
    report_started: Option<DateTime<Utc>>,
    report_ended: Option<DateTime<Utc>>,
}
