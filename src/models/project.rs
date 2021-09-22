use chrono::{DateTime, Utc};
use sqlx::{
    postgres::{PgArgumentBuffer, PgValueRef},
    query, query_as, Decode, Encode, FromRow, PgPool, Postgres, Type,
};
use uuid::Uuid;

use crate::view::project::NewProject;

use super::customer::CustomerId;

pub type ProjectId = Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ProjectEntity {
    pub id: ProjectId,
    pub customer_id: CustomerId,
    pub name: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ProjectEntity {
    pub async fn new(pool: &PgPool, new_project: NewProject) -> Result<Self, sqlx::Error> {
        Ok(query_as!(
            ProjectEntity,
            r#"INSERT INTO projects (
                customer_id,
                name,
                note
            )
            VALUES ($1,$2,$3)
            RETURNING *;"#,
            new_project.customer_id,
            new_project.name,
            new_project.note,
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn count_projects(pool: &PgPool) -> Result<i64, sqlx::Error> {
        Ok(query!(
            r#"SELECT COUNT(id) 
            FROM projects;"#
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0))
    }

    pub async fn get_project_by_id(pool: &PgPool, id: ProjectId) -> Result<Self, sqlx::Error> {
        Ok(query_as!(
            ProjectEntity,
            r#"SELECT *
            FROM projects
            WHERE id = $1;"#,
            id
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_projects_for_customer_id(
        pool: &PgPool,
        id: CustomerId,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(query_as!(
            ProjectEntity,
            r#"SELECT *
            FROM projects
            WHERE customer_id = $1;"#,
            id
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn list_projects(
        pool: &PgPool,
        start: i64,
        limit: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(query_as!(
            ProjectEntity,
            r#"SELECT *
            FROM projects
            LIMIT $1
            OFFSET $2;"#,
            limit,
            start
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn delete_project(pool: &PgPool, id: ProjectId) -> Result<Self, sqlx::Error> {
        Ok(query_as!(
            ProjectEntity,
            r#"DELETE
            FROM projects
            WHERE id = $1
            RETURNING *;"#,
            id
        )
        .fetch_one(pool)
        .await?)
    }
}

impl<'r> Decode<'r, Postgres> for ProjectEntity {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        Ok(Self {
            id: decoder.try_decode::<ProjectId>()?,
            customer_id: decoder.try_decode::<CustomerId>()?,
            name: decoder.try_decode::<String>()?,
            note: decoder.try_decode::<Option<String>>()?,
            created_at: decoder.try_decode::<DateTime<Utc>>()?,
            updated_at: decoder.try_decode::<DateTime<Utc>>()?,
        })
    }
}

impl<'r> Encode<'r, Postgres> for ProjectEntity {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        let mut encoder = sqlx::postgres::types::PgRecordEncoder::new(buf);
        encoder.encode(&self.id);
        encoder.encode(&self.customer_id);
        encoder.encode(&self.name);
        encoder.encode(&self.note);
        encoder.encode(&self.updated_at);
        encoder.encode(&self.created_at);
        encoder.finish();
        sqlx::encode::IsNull::No
    }
}
