use async_graphql::{validators::IntGreaterThan, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        intern_merchandise::{
            InternMerchandise as DbInternMerchandise, InternMerchandiseId, InternMerchandiseStatus,
        },
        users::UserId,
    },
    validators::Url,
};

#[derive(Serialize, Debug, Clone, SimpleObject)]
pub struct InternMerchandise {
    pub id: InternMerchandiseId,
    pub merchandise_id: Option<i64>,
    pub orderer_id: UserId,
    pub project_leader_id: UserId,
    pub purchased_on: DateTime<Utc>,
    pub count: i64,
    pub cost: f32,
    pub status: InternMerchandiseStatus,
    pub merchandise_name: String,
    pub use_case: Option<String>,
    pub location: Option<String>,
    pub article_number: String,
    pub shop: String,
    pub serial_number: Option<String>,
    pub arrived_on: Option<DateTime<Utc>>,
    pub url: Option<String>,
    pub postage: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<DbInternMerchandise> for InternMerchandise {
    fn from(merch: DbInternMerchandise) -> Self {
        let postage = match merch.postage {
            None => None,
            Some(r) => Some(r.to_string().parse::<f32>().unwrap_or(0.0)),
        };
        Self {
            id: merch.id,
            merchandise_id: merch.merchandise_id,
            orderer_id: merch.orderer_id,
            project_leader_id: merch.project_leader_id,
            purchased_on: merch.purchased_on,
            count: merch.count,
            cost: merch.cost.to_string().parse::<f32>().unwrap_or(0.0f32),
            status: merch.status,
            merchandise_name: merch.merchandise_name,
            use_case: merch.use_case,
            location: merch.location,
            article_number: merch.article_number,
            shop: merch.shop,
            serial_number: merch.serial_number,
            arrived_on: merch.arrived_on,
            url: merch.url,
            postage,
            created_at: merch.created_at,
            updated_at: merch.updated_at,
        }
    }
}

#[derive(InputObject, Deserialize, Serialize)]
pub struct NewInternMerchandise {
    pub merchandise_name: String,
    #[graphql(validator(IntGreaterThan(value = "0")))]
    pub count: i64,
    #[graphql(validator(Url))]
    pub url: Option<String>,
    pub project_leader_id: UserId,
    pub article_number: String,
    pub cost: f32,
    pub postage: f32,
    pub use_case: Option<String>,
    pub location: Option<String>,
    pub shop: String,
}

#[derive(InputObject, Deserialize, Serialize)]
pub struct IncomingInternMerchandise {
    pub id: InternMerchandiseId,
    pub merchandise_id: i64,
    pub serial_number: String,
}
