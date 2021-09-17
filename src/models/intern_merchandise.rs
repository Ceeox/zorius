use std::str::FromStr;

use async_graphql::Enum;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgArgumentBuffer, query, query_as, types::Decimal, Decode, Encode, FromRow, PgPool,
    Postgres, Type,
};
use uuid::Uuid;

use crate::{
    models::users::{UserEntity, UserId},
    view::intern_merchandise::{IncomingInternMerchandise, NewInternMerchandise},
};

pub type InternMerchandiseId = Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct InternMerchandiseEntity {
    pub id: InternMerchandiseId,
    pub merchandise_id: Option<i64>,
    pub orderer_id: UserId,
    pub project_leader_id: UserId,
    pub purchased_on: DateTime<Utc>,
    pub count: i64,
    pub cost: Decimal,
    pub status: InternMerchandiseStatus,
    pub merchandise_name: String,
    pub use_case: Option<String>,
    pub location: Option<String>,
    pub article_number: String,
    pub shop: String,
    pub serial_number: Option<String>,
    pub arrived_on: Option<DateTime<Utc>>,
    pub url: Option<String>,
    pub postage: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl InternMerchandiseEntity {
    pub async fn new(
        pool: &PgPool,
        orderer_id: UserId,
        new_intern_merch: NewInternMerchandise,
    ) -> Result<InternMerchandiseEntity, sqlx::Error> {
        let res = query_as!(
            InternMerchandiseEntity,
            r#"INSERT INTO intern_merchandises (
                merchandise_id,
                orderer_id,
                project_leader_id,
                purchased_on,
                count,
                cost,
                status,
                merchandise_name,
                use_case,
                location,
                article_number,
                shop,
                serial_number,
                url,
                postage
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)
            RETURNING
                id,
                merchandise_id,
                orderer_id,
                project_leader_id,
                purchased_on,
                count,
                cost,
                status as "status:_",
                merchandise_name,
                use_case,
                location,
                article_number,
                shop,
                serial_number,
                arrived_on,
                url,
                postage,
                created_at,
                updated_at;"#,
            None as Option<i64>,
            orderer_id,
            new_intern_merch.project_leader_id,
            Utc::now().into(),
            new_intern_merch.count,
            Decimal::from_str(&new_intern_merch.cost.to_string()).unwrap_or(Decimal::default()),
            InternMerchandiseStatus::Ordered as _,
            new_intern_merch.merchandise_name,
            new_intern_merch.use_case,
            new_intern_merch.location,
            new_intern_merch.article_number,
            new_intern_merch.shop,
            None as Option<String>,
            new_intern_merch.url,
            Decimal::from_str(&new_intern_merch.postage.to_string()).unwrap_or(Decimal::default())
        )
        .fetch_one(pool)
        .await?;

        Ok(res)
    }

    pub async fn get_intern_merch_by_id(
        pool: &PgPool,
        id: InternMerchandiseId,
    ) -> Result<InternMerchandiseEntity, sqlx::Error> {
        Ok(sqlx::query_as!(
            InternMerchandiseEntity,
            r#"SELECT 
                id,
                merchandise_id,
                orderer_id,
                project_leader_id,
                purchased_on,
                count,
                cost,
                status as "status: _",
                merchandise_name,
                use_case,
                location,
                article_number,
                shop,
                serial_number,
                arrived_on,
                url,
                postage,
                created_at,
                updated_at
            FROM intern_merchandises
            WHERE id = $1
            ORDER BY created_at DESC;"#,
            id
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn list_intern_merch(
        pool: &PgPool,
        start: i64,
        limit: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(query_as!(
            InternMerchandiseEntity,
            r#"SELECT
                    id,
                    merchandise_id,
                    orderer_id,
                    project_leader_id,
                    purchased_on,
                    count,
                    cost,
                    status as "status: _",
                    merchandise_name,
                    use_case,
                    location,
                    article_number,
                    shop,
                    serial_number,
                    arrived_on,
                    url,
                    postage,
                    created_at,
                    updated_at
                FROM intern_merchandises
                ORDER BY created_at ASC
                LIMIT $1
                OFFSET $2;"#,
            limit,
            start
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn count_intern_merch(pool: &PgPool) -> Result<i64, sqlx::Error> {
        Ok(sqlx::query!(
            r#"SELECT COUNT(id)
            FROM intern_merchandises;"#
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or_default())
    }

    pub async fn incoming_intern_merchandise(
        pool: &PgPool,
        update: IncomingInternMerchandise,
    ) -> Result<InternMerchandiseEntity, sqlx::Error> {
        Ok(query_as!(
            Self,
            r#"UPDATE intern_merchandises
                SET 
                    merchandise_id = $2,
                    serial_number = $3,
                    arrived_on = NOW()
                WHERE id = $1
                RETURNING
                    id,
                    merchandise_id,
                    orderer_id,
                    project_leader_id,
                    purchased_on,
                    count,
                    cost,
                    status as "status: _",
                    merchandise_name,
                    use_case,
                    location,
                    article_number,
                    shop,
                    serial_number,
                    arrived_on,
                    url,
                    postage,
                    created_at,
                    updated_at;"#,
            update.id,
            update.merchandise_id,
            update.serial_number,
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        query!(
            r#"DELETE
            FROM intern_merchandises
            WHERE id = $1;"#,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

impl<'r> Decode<'r, Postgres> for InternMerchandiseEntity {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;

        Ok(InternMerchandiseEntity {
            id: decoder.try_decode::<InternMerchandiseId>()?,
            merchandise_id: decoder.try_decode::<Option<i64>>()?,
            orderer_id: decoder.try_decode::<UserId>()?,
            project_leader_id: decoder.try_decode::<UserId>()?,
            purchased_on: decoder.try_decode::<DateTime<Utc>>()?,
            count: decoder.try_decode::<i64>()?,
            cost: decoder.try_decode::<Decimal>()?,
            status: decoder.try_decode::<InternMerchandiseStatus>()?,
            merchandise_name: decoder.try_decode::<String>()?,
            use_case: decoder.try_decode::<Option<String>>()?,
            location: decoder.try_decode::<Option<String>>()?,
            article_number: decoder.try_decode::<String>()?,
            shop: decoder.try_decode::<String>()?,
            serial_number: decoder.try_decode::<Option<String>>()?,
            arrived_on: decoder.try_decode::<Option<DateTime<Utc>>>()?,
            url: decoder.try_decode::<Option<String>>()?,
            postage: decoder.try_decode::<Option<Decimal>>()?,
            created_at: decoder.try_decode::<DateTime<Utc>>()?,
            updated_at: decoder.try_decode::<DateTime<Utc>>()?,
        })
    }
}

impl<'r> Encode<'r, Postgres> for InternMerchandiseEntity {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        let mut encoder = sqlx::postgres::types::PgRecordEncoder::new(buf);

        encoder.encode(self.id);
        encoder.encode(self.merchandise_id);
        encoder.encode(self.orderer_id);
        encoder.encode(self.project_leader_id);
        encoder.encode(self.purchased_on);
        encoder.encode(self.count);
        encoder.encode(self.cost);
        encoder.encode(self.status);
        encoder.encode(self.merchandise_name.clone());
        encoder.encode(self.use_case.clone());
        encoder.encode(self.location.clone());
        encoder.encode(self.article_number.clone());
        encoder.encode(self.shop.clone());
        encoder.encode(self.serial_number.clone());
        encoder.encode(self.arrived_on);
        encoder.encode(self.url.clone());
        encoder.encode(self.postage);
        encoder.finish();
        sqlx::encode::IsNull::No
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Type, Enum)]
#[sqlx(type_name = "intern_merchandise_status", rename_all = "snake_case")]
pub enum InternMerchandiseStatus {
    Ordered,
    Delivered,
    Stored,
    Used,
}

impl Default for InternMerchandiseStatus {
    fn default() -> Self {
        InternMerchandiseStatus::Ordered
    }
}

// #[derive(Template)]
// #[template(path = "intern_merch_used.html")]
// pub struct StatusTemplate {
//     pub(crate) id: InternMerchandiseId,
//     pub(crate) merchandise_id: Option<i32>,
//     pub(crate) orderer_name: String,
//     pub(crate) count: i32,
//     pub(crate) merchandise_name: String,
//     pub(crate) cost: String,
//     pub(crate) status: InternMerchandiseStatus,
// }
