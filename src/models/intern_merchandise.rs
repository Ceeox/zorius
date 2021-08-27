use std::{fmt::Display, str::FromStr};

use async_graphql::Enum;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgArgumentBuffer, query_file_as, types::Decimal, Decode, Encode, FromRow, PgPool,
    Postgres, Type,
};
use uuid::Uuid;

use crate::{models::users::UserId, view::intern_merchandise::NewInternMerchandise};

pub type InternMerchandiseId = Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct InternMerchandise {
    pub id: InternMerchandiseId,
    pub merchandise_id: Option<i64>,
    pub orderer_id: UserId,
    pub project_leader_id: UserId,
    pub purchased_on: DateTime<Utc>,
    pub count: i64,
    pub cost: Decimal,
    pub merch_status: InternMerchandiseStatus,
    pub merchandise_name: String,
    pub use_case: Option<String>,
    pub location: Option<String>,
    pub article_number: String,
    pub shop: String,
    pub serial_number: Option<String>,
    pub arrived_on: Option<DateTime<Utc>>,
    pub url: Option<String>,
    pub postage: Option<Decimal>,
    pub invoice_number: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> Decode<'r, Postgres> for InternMerchandise {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;

        Ok(InternMerchandise {
            id: decoder.try_decode::<InternMerchandiseId>()?,
            merchandise_id: decoder.try_decode::<Option<i64>>()?,
            orderer_id: decoder.try_decode::<UserId>()?,
            project_leader_id: decoder.try_decode::<UserId>()?,
            purchased_on: decoder.try_decode::<DateTime<Utc>>()?,
            count: decoder.try_decode::<i64>()?,
            merchandise_name: decoder.try_decode::<String>()?,
            use_case: decoder.try_decode::<Option<String>>()?,
            location: decoder.try_decode::<Option<String>>()?,
            article_number: decoder.try_decode::<String>()?,
            shop: decoder.try_decode::<String>()?,
            cost: decoder.try_decode::<Decimal>()?,
            serial_number: decoder.try_decode::<Option<String>>()?,
            arrived_on: decoder.try_decode::<Option<DateTime<Utc>>>()?,
            merch_status: decoder.try_decode::<InternMerchandiseStatus>()?,
            url: decoder.try_decode::<Option<String>>()?,
            postage: decoder.try_decode::<Option<Decimal>>()?,
            invoice_number: decoder.try_decode::<Option<i64>>()?,
            created_at: decoder.try_decode::<DateTime<Utc>>()?,
            updated_at: decoder.try_decode::<DateTime<Utc>>()?,
        })
    }
}

impl<'r> Encode<'r, Postgres> for InternMerchandise {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        let mut encoder = sqlx::postgres::types::PgRecordEncoder::new(buf);

        encoder.encode(self.id);
        encoder.encode(self.merchandise_id);
        encoder.encode(self.orderer_id);
        encoder.encode(self.project_leader_id);
        encoder.encode(self.purchased_on);
        encoder.encode(self.count);
        encoder.encode(self.cost);
        encoder.encode(self.merch_status);
        encoder.encode(self.merchandise_name.clone());
        encoder.encode(self.use_case.clone());
        encoder.encode(self.location.clone());
        encoder.encode(self.article_number.clone());
        encoder.encode(self.shop.clone());
        encoder.encode(self.serial_number.clone());
        encoder.encode(self.arrived_on);
        encoder.encode(self.url.clone());
        encoder.encode(self.postage);
        encoder.encode(self.invoice_number);
        encoder.encode(self.created_at);
        encoder.encode(self.updated_at);
        encoder.finish();
        sqlx::encode::IsNull::No
    }
}

impl InternMerchandise {
    pub fn new(new_intern_merchandise: NewInternMerchandise) -> Self {
        Self {
            id: InternMerchandiseId::new_v4(),
            merchandise_name: new_intern_merchandise.merchandise_name,
            count: new_intern_merchandise.count,
            orderer_id: new_intern_merchandise.orderer_id,
            purchased_on: Utc::now().into(),
            cost: Decimal::from_str(&new_intern_merchandise.cost.to_string())
                .unwrap_or(Decimal::default()),
            merch_status: InternMerchandiseStatus::Ordered,
            url: new_intern_merchandise.url,
            use_case: new_intern_merchandise.use_case,
            article_number: new_intern_merchandise.article_number,
            postage: Some(
                Decimal::from_str(&new_intern_merchandise.cost.to_string())
                    .unwrap_or(Decimal::default()),
            ),
            project_leader_id: new_intern_merchandise.project_leader_id,
            location: new_intern_merchandise.location,
            shop: new_intern_merchandise.shop,

            merchandise_id: None,
            serial_number: None,
            arrived_on: None,
            invoice_number: None,
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        }
    }

    // pub fn change_status(&mut self, new_status: InternMerchandiseStatus, user: User) {
    //     self.status = new_status;
    //     self.updated_date = Utc::now().into();
    //     let orderer_name = format!("{} {}", user.firstname, user.lastname);
    //     let template: StatusTemplate = StatusTemplate {
    //         id: self.id.clone(),
    //         merchandise_id: self.merchandise_id,
    //         orderer_name,
    //         count: self.count,
    //         merchandise_name: self.merchandise_name.clone(),
    //         cost: self.cost,
    //         status: new_status,
    //     };
    //     let body = template.render().unwrap();

    //     mailer(
    //         &CONFIG.mailer.merchandise_email_send_to,
    //         &format!(
    //             "Intern Merchandise Staus Change to {} for {}",
    //             new_status.to_string(),
    //             self.merchandise_name,
    //         ),
    //         &body,
    //     );
    // }

    // pub async fn get_intern_merch_by_id(
    //     pool: &PgPool,
    //     id: InternMerchandiseId,
    // ) -> Result<Self, sqlx::Error> {
    //     Ok(
    //         sqlx::query_file_as!(Self, "sql/get_intern_merch_by_id.sql", id)
    //             .fetch_one(pool)
    //             .await?,
    //     )
    // }

    // pub async fn get_intern_merch_by_merch_id(
    //     &self,
    //     merchandise_id: i32,
    // ) -> Result<InternMerchandise> {
    //     let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
    //     let pipeline = AggregateBuilder::new()
    //         .matching(vec![("merchandise_id", merchandise_id)])
    //         .lookup(MDB_COLL_NAME_USERS, "orderer_id", "_id", "orderer")
    //         .lookup(
    //             MDB_COLL_NAME_USERS,
    //             "project_leader_id",
    //             "_id",
    //             "project_leader",
    //         )
    //         .unwind("$orderer", None, None)
    //         .unwind("$project_leader", None, None)
    //         .build();
    //     let mut doc = collection.aggregate(pipeline, None).await?;
    //     match doc.next().await {
    //         Some(r) => Ok(from_document(r?)?),
    //         None => Err(Error::new("intern merch wasn't found")),
    //     }
    // }

    // pub async fn list_intern_merch(&self, start: i64, limit: i64) -> Result<Cursor<Document>> {
    //     let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
    //     let pipeline = AggregateBuilder::new()
    //         .skip(start as i64)
    //         .limit(limit)
    //         .sort(vec![("created_date", SortOrder::DESC)])
    //         .lookup(MDB_COLL_NAME_USERS, "orderer_id", "_id", "orderer")
    //         .lookup(
    //             MDB_COLL_NAME_USERS,
    //             "project_leader_id",
    //             "_id",
    //             "project_leader",
    //         )
    //         .unwind("$orderer", None, None)
    //         .unwind("$project_leader", None, None)
    //         .build();
    //     Ok(collection.aggregate(pipeline, None).await?)
    // }

    // pub async fn list_intern_merch(
    //     pool: &PgPool,
    //     start: i64,
    //     limit: i64,
    // ) -> Result<Vec<Self>, sqlx::Error> {
    //     Ok(
    //         query_file_as!(Self, "sql/list_intern_merch.sql", limit, start)
    //             .fetch_all(pool)
    //             .await?,
    //     )
    // }

    pub async fn count_intern_merch(pool: &PgPool) -> Result<i64, sqlx::Error> {
        Ok(sqlx::query_file!("sql/count_intern_merch.sql")
            .fetch_one(pool)
            .await?
            .count
            .unwrap_or(0))
    }

    // pub async fn new_intern_merch(
    //     pool: &PgPool,
    //     new_intern_merch: NewInternMerchandise,
    // ) -> Result<InternMerchandise, sqlx::Error> {
    //     let new_intern_merch = InternMerchandise::new(new_intern_merch);
    //     // let id = new_intern_merch.id.clone();
    //     let intern_merch = query_file_as!(
    //         InternMerchandise,
    //         "sql/new_intern_merch.sql",
    //         new_intern_merch.id,
    //         new_intern_merch.merchandise_id,
    //         new_intern_merch.orderer_id,
    //         new_intern_merch.project_leader_id,
    //         new_intern_merch.purchased_on,
    //         new_intern_merch.count,
    //         new_intern_merch.cost,
    //         new_intern_merch.status.to_string(),
    //         new_intern_merch.merchandise_name,
    //         new_intern_merch.use_case,
    //         new_intern_merch.location,
    //         new_intern_merch.article_number,
    //         new_intern_merch.shop,
    //         new_intern_merch.serial_number,
    //         new_intern_merch.arrived_on,
    //         new_intern_merch.url,
    //         new_intern_merch.postage,
    //         new_intern_merch.invoice_number,
    //         new_intern_merch.created_at,
    //         new_intern_merch.updated_at,
    //     )
    //     .fetch_one(pool)
    //     .await?;

    //     Ok(intern_merch)
    // }

    // pub async fn update_intern_merch(
    //     &self,
    //     id: InternMerchandiseId,
    //     update: InternMerchandiseUpdate,
    // ) -> Result<InternMerchandise> {
    //     let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
    //     let filter = doc! {"_id": id.clone()};
    //     let update = doc! {"$set": bson::to_bson(&update)?};
    //     let _ = collection.update_one(filter, update, None).await?;
    //     Ok(self.get_intern_merch_by_id(id).await?)
    // }

    // pub async fn delete_intern_merch(&self, id: InternMerchandiseId) -> Result<()> {
    //     let col = self.database.collection(MDB_COLL_INTERN_MERCH);
    //     let query = doc! {"_id": id};
    //     let _ = col.delete_one(query, None).await?;
    //     Ok(())
    // }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Type, Enum)]
#[sqlx(type_name = "intern_merchandise_status", rename_all = "snake_case")]
pub enum InternMerchandiseStatus {
    Ordered,
    Delivered,
    Stored,
    Used,
}

impl Display for InternMerchandiseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternMerchandiseStatus::Ordered => write!(f, "Ordered"),
            InternMerchandiseStatus::Delivered => write!(f, "Delivered"),
            InternMerchandiseStatus::Stored => write!(f, "Stored"),
            InternMerchandiseStatus::Used => write!(f, "Used"),
        }
    }
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
