use chrono::{DateTime, Utc};
use sqlx::{query_as, FromRow, PgPool};
use uuid::Uuid;

use crate::{
    models::{customer::CustomerId, project::ProjectId, users::UserId},
    view::work_report::NewWorkReport,
};

pub type WorkReportId = Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct WorkReport {
    id: WorkReportId,
    owner_id: UserId,
    customer_id: CustomerId,
    project_id: Option<ProjectId>,
    description: String,
    invoiced: bool,
    report_started: DateTime<Utc>,
    report_ended: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl WorkReport {
    pub async fn new(
        pool: &PgPool,
        owner_id: UserId,
        new_wr: NewWorkReport,
    ) -> Result<Self, sqlx::Error> {
        Ok(query_as!(
            WorkReport,
            r#"INSERT INTO work_reports (
                owner_id,
                customer_id,
                project_id,
                invoiced,
                description
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5
            )
            RETURNING *;"#,
            owner_id,
            new_wr.customer_id,
            new_wr.project_id,
            new_wr.invoiced,
            new_wr.description
        )
        .fetch_one(pool)
        .await?)
    }
}
