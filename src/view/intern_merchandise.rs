use std::fmt::Display;

use async_graphql::{validators::IntGreaterThan, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        intern_merchandise::{InternMerchandiseId, InternMerchandiseStatus},
        users::UserId,
    },
    validators::Url,
    view::users::User,
};

#[derive(Serialize, Debug, Clone, SimpleObject)]
pub struct InternMerchandise {
    pub id: InternMerchandiseId,
    pub merchandise_id: Option<i32>,
    pub orderer: User,
    pub project_leader: User,
    pub purchased_on: DateTime<Utc>,
    pub count: i64,
    pub cost: f32,
    pub status: InternMerchandiseStatus,
    pub merchandise_name: String,
    pub use_case: String,
    pub location: String,
    pub article_number: String,
    pub shop: String,
    pub serial_number: String,
    pub arrived_on: Option<DateTime<Utc>>,
    pub url: String,
    pub postage: Option<f32>,
    pub invoice_number: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(InputObject, Deserialize, Serialize)]
pub struct NewInternMerchandise {
    pub merchandise_name: String,
    #[graphql(validator(IntGreaterThan(value = "0")))]
    pub count: i64,
    #[graphql(validator(Url))]
    pub url: Option<String>,
    pub orderer_id: UserId,
    pub project_leader_id: UserId,
    pub article_number: String,
    pub cost: f32,
    pub postage: f32,
    pub use_case: Option<String>,
    pub location: Option<String>,
    pub shop: String,
}

#[derive(Deserialize, Serialize, Debug, InputObject, Default)]
pub struct InternMerchandiseUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderer_id: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_leader_id: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased_on: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrived_on: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postage: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<i32>,
}
