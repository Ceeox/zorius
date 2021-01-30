use async_graphql::{Enum, InputObject, SimpleObject};
use bson::{oid::ObjectId, DateTime};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::models::{
    user::UserId,
    work_report::{customer::CustomerId, project::ProjectId},
};

pub(crate) mod customer;
pub(crate) mod project;

pub type WorkReportId = ObjectId;
#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
pub struct WorkReport {
    #[serde(rename = "_id")]
    id: ObjectId,
    user_id: UserId,
    customer_id: CustomerId,
    project_id: ProjectId,
    trip_info: TripInfo,
    description: String,
    started: DateTime,
    ended: Option<DateTime>,
    status: WorkReportStatus,
    invoiced: bool,
}

#[derive(Deserialize, Debug, InputObject)]
pub struct NewWorkReport {
    pub customer_id: CustomerId,
    pub project_id: ProjectId,
    pub to_customer_started: Option<DateTime>,
    pub to_customer_arrived: Option<DateTime>,
    pub from_customer_started: Option<DateTime>,
    pub from_customer_arrived: Option<DateTime>,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, InputObject, Clone)]
pub struct WorkReportUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_id: Option<CustomerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<ProjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_customer_started: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_customer_arrived: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_customer_started: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_customer_arrived: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    started: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ended: Option<Option<DateTime>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<WorkReportStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoiced: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
pub struct TripInfo {
    to_customer_started: Option<DateTime>,
    to_customer_arrived: Option<DateTime>,

    from_customer_started: Option<DateTime>,
    from_customer_arrived: Option<DateTime>,
}

#[derive(Serialize, Deserialize, Debug, Enum, PartialEq, Eq, Clone, Copy)]
pub enum WorkReportStatus {
    Finished,
    Paused,
    Running,
}

impl WorkReport {
    pub fn new(user_id: UserId, new_wr: NewWorkReport) -> Self {
        Self {
            id: ObjectId::new(),
            user_id: user_id,
            customer_id: new_wr.customer_id,
            project_id: new_wr.project_id,
            trip_info: TripInfo {
                to_customer_started: new_wr.to_customer_started,
                to_customer_arrived: new_wr.to_customer_arrived,
                from_customer_started: new_wr.from_customer_started,
                from_customer_arrived: new_wr.from_customer_arrived,
            },
            description: new_wr.description,
            started: Utc::now().into(),
            ended: None,
            status: WorkReportStatus::Running,
            invoiced: true,
        }
    }
}
