use chrono::{DateTime, Utc};
use sqlx::{query_as, FromRow, PgPool};
use uuid::Uuid;

use crate::{
    models::{customer::CustomerId, project::ProjectId, users::UserId},
    view::work_report::{NewWorkReport, WorkReportUpdate},
};

pub type WorkReportId = Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct WorkReportEntity {
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

impl WorkReportEntity {
    pub async fn new(
        pool: &PgPool,
        owner_id: UserId,
        new_wr: NewWorkReport,
    ) -> Result<Self, sqlx::Error> {
        Ok(query_as!(
            WorkReportEntity,
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

    pub async fn work_report_by_id(pool: &PgPool, id: WorkReportId) -> Result<Self, sqlx::Error> {
        Ok(query_as!(
            WorkReportEntity,
            r#"SELECT *
            FROM work_reports
            WHERE id = $1"#,
            id
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn list_work_reports(
        pool: &PgPool,
        start: i64,
        limit: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(query_as!(
            WorkReportEntity,
            r#"SELECT *
            FROM work_reports
            ORDER BY created_at ASC
            LIMIT $1
            OFFSET $2;"#,
            limit,
            start
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn update_work_report(
        pool: &PgPool,
        id: WorkReportId,
        update: WorkReportUpdate,
    ) -> Result<Vec<Self>, sqlx::Error> {
        todo!()
    }
}
