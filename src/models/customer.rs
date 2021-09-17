use chrono::{DateTime, Utc};
use sqlx::{query, query_as, FromRow, PgPool};
use uuid::Uuid;

use crate::{
    models::project::ProjectEntity,
    view::customer::{Customer as CustomerView, NewCustomer},
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

    pub async fn get_customer_by_id(pool: &PgPool, id: CustomerId) -> Result<Self, sqlx::Error> {
        let res = query_as!(
            CustomerEntity,
            "SELECT *
            FROM customers
            WHERE id = $1",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(res)
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

    // pub async fn update_customer(
    //     pool: &PgPool,
    //     id: CustomerId,
    //     update: UpdateCustomer,
    // ) -> Result<CustomerView, sqlx::Error> {
    //     Ok(query_as!(
    //         CustomerView,
    //         r#"WITH updated as (
    //                 UPDATE customers
    //                 SET name = $2,
    //                     identifier = $3,
    //                     note = $4
    //                 WHERE id = $1
    //                 RETURNING *
    //             )
    //             SELECT updated.*, projects.*
    //             FROM updated, projects
    //             WHERE updated.id = projects.customer_id;"#,
    //         id,
    //         update.name,
    //         update.identifier,
    //         update.note.unwrap_or(None)
    //     )
    //     .fetch_one(pool)
    //     .await?)
    // }

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
