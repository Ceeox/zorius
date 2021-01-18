use async_graphql::{Enum, InputObject, SimpleObject};
use bson::{oid::ObjectId, DateTime};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use super::{customer::CustomerId, project::ProjectId, user::UserId};

pub type WorkReportId = ObjectId;
#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
pub struct WorkReport {
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
