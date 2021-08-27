use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{
        customer::{Customer, CustomerId},
        project::{Project, ProjectId},
        users::UserId,
    },
    view::users::User,
};

pub type WorkReportId = Uuid;

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
pub struct DBWorkReport {
    id: WorkReportId,
    user_id: UserId,
    customer_id: CustomerId,
    project_id: ProjectId,
    trip_info: TripInfo,
    description: String,
    times: Vec<WorkReportTimes>,
    status: WorkReportStatus,
    invoiced: bool,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
pub struct WorkReportTimes {
    pub started: DateTime<Utc>,
    pub ended: Option<DateTime<Utc>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct WorkReport {
    id: WorkReportId,
    user: User,
    customer: Customer,
    project: Project,
    trip_info: TripInfo,
    description: String,
    times: Vec<WorkReportTimes>,
    status: WorkReportStatus,
    invoiced: bool,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Debug, InputObject)]
pub struct NewWorkReport {
    pub customer_id: CustomerId,
    pub project_id: ProjectId,
    pub to_customer_started: Option<DateTime<Utc>>,
    pub to_customer_arrived: Option<DateTime<Utc>>,
    pub from_customer_started: Option<DateTime<Utc>>,
    pub from_customer_arrived: Option<DateTime<Utc>>,
    pub description: String,
}

#[derive(Serialize, Debug, InputObject, Clone)]
pub struct WorkReportUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_id: Option<CustomerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<ProjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_customer_started: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_customer_arrived: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_customer_started: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_customer_arrived: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    started: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ended: Option<Option<DateTime<Utc>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<WorkReportStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoiced: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
pub struct TripInfo {
    to_customer_started: Option<DateTime<Utc>>,
    to_customer_arrived: Option<DateTime<Utc>>,

    from_customer_started: Option<DateTime<Utc>>,
    from_customer_arrived: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Enum, PartialEq, Eq, Clone, Copy)]
pub enum WorkReportStatus {
    Finished,
    Paused,
    Running,
}

impl DBWorkReport {
    pub fn new(user_id: UserId, new_wr: NewWorkReport) -> Self {
        Self {
            id: WorkReportId::new_v4(),
            user_id,
            customer_id: new_wr.customer_id,
            project_id: new_wr.project_id,
            trip_info: TripInfo {
                to_customer_started: new_wr.to_customer_started,
                to_customer_arrived: new_wr.to_customer_arrived,
                from_customer_started: new_wr.from_customer_started,
                from_customer_arrived: new_wr.from_customer_arrived,
            },
            description: new_wr.description,
            times: vec![WorkReportTimes {
                started: Utc::now().into(),
                ended: None,
            }],
            status: WorkReportStatus::Running,
            invoiced: true,
            created_at: Utc::now().into(),
        }
    }

    pub fn get_id(&self) -> &WorkReportId {
        &self.id
    }
}
