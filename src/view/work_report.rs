use async_graphql::InputObject;
use serde::Serialize;

use crate::models::{customer::CustomerId, project::ProjectId};

#[derive(Serialize, Debug, InputObject)]
pub struct NewWorkReport {
    pub customer_id: CustomerId,
    pub project_id: Option<ProjectId>,
    pub description: String,
    pub invoiced: bool,
}
