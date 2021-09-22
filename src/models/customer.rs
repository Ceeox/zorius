use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, query, query_as, PgPool, Row};
use uuid::Uuid;

use crate::{
    models::project::ProjectEntity,
    view::{
        customer::{Customer, NewCustomer, UpdateCustomer},
        project::Project,
    },
};

pub type CustomerId = Uuid;

#[derive(Debug, Clone)]
pub struct CustomerEntity {
    pub id: CustomerId,
    pub name: String,
    pub identifier: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CustomerEntity {
    pub async fn new(pool: &PgPool, new_customer: NewCustomer) -> Result<Self, sqlx::Error> {
        let res = query_as!(
            CustomerEntity,
            r#"INSERT INTO customers (
                name,
                identifier,
                note
            )
            VALUES ($1,$2,$3)
            RETURNING *;"#,
            new_customer.name,
            new_customer.identifier,
            new_customer.note,
        )
        .fetch_one(pool)
        .await?;
        Ok(res)
    }

    pub async fn get_customer_by_id(
        pool: &PgPool,
        id: CustomerId,
    ) -> Result<Customer, sqlx::Error> {
        let res = query_as!(
            CustomerEntity,
            r#"SELECT *
            FROM customers
            WHERE id = $1"#,
            id
        )
        .fetch_one(pool)
        .await?;

        let projects: Vec<ProjectEntity> = query_as!(
            ProjectEntity,
            r#"SELECT *
            FROM projects
            WHERE customer_id = $1;"#,
            id
        )
        .fetch_all(pool)
        .await?;

        let projects = projects
            .into_iter()
            .map(|p| p.into())
            .collect::<Vec<Project>>();

        Ok(Customer {
            id: res.id,
            name: res.name,
            identifier: res.identifier,
            note: res.note,
            projects,
            created_at: res.created_at,
            updated_at: res.updated_at,
        })
    }

    pub async fn count_customers(pool: &PgPool) -> Result<i64, sqlx::Error> {
        Ok(query!(
            r#"SELECT COUNT(id) 
            FROM customers;"#
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0))
    }

    pub async fn list_customer(
        pool: &PgPool,
        start: i64,
        limit: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(query_as!(
            CustomerEntity,
            r#"SELECT *
            FROM customers
            ORDER BY created_at ASC
            LIMIT $1
            OFFSET $2;"#,
            limit,
            start,
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn update_customer(
        pool: &PgPool,
        id: CustomerId,
        update: UpdateCustomer,
    ) -> Result<Customer, sqlx::Error> {
        query_as!(
            Customer,
            r#"UPDATE customers
            SET name = $2, identifier = $3, note = $4
            WHERE id = $1;"#,
            id,
            update.name,
            update.identifier,
            update.note.unwrap_or(None)
        )
        .fetch_one(pool)
        .await?;
        Ok(CustomerEntity::get_customer_by_id(pool, id).await?)
    }

    pub async fn delete_customer(pool: &PgPool, id: CustomerId) -> Result<Self, sqlx::Error> {
        Ok(query_as!(
            CustomerEntity,
            r#"DELETE
            FROM customers
            WHERE id = $1
            RETURNING *;"#,
            id
        )
        .fetch_one(pool)
        .await?)
    }
}
