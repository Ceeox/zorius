use async_graphql::{validators::IntGreaterThan, InputObject, SimpleObject};
use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{intern_merchandise::Model, users},
    validators::Url,
    view::users::User,
};

#[derive(Serialize, Debug, Clone, SimpleObject)]
pub struct InternMerchandise {
    pub id: Uuid,
    pub merchandise_id: Option<i64>,
    pub orderer: Option<User>,
    pub controller: Option<User>,
    pub project_leader: Option<User>,
    pub purchased_on: DateTime<FixedOffset>,
    pub count: i64,
    pub cost: Decimal,
    pub merchandise_name: String,
    pub use_case: Option<String>,
    pub location: Option<String>,
    pub article_number: String,
    pub shop: String,
    pub serial_number: Option<String>,
    pub arrived_on: Option<DateTime<FixedOffset>>,
    pub url: Option<String>,
    pub postage: Option<Decimal>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl
    From<(
        Model,
        (
            Option<users::Model>,
            Option<users::Model>,
            Option<users::Model>,
        ),
    )> for InternMerchandise
{
    fn from(
        (merch, (orderer, controller, project_leader)): (
            Model,
            (
                Option<users::Model>,
                Option<users::Model>,
                Option<users::Model>,
            ),
        ),
    ) -> Self {
        Self {
            id: merch.id,
            merchandise_id: merch.merchandise_id,
            orderer: if let Some(o) = orderer {
                Some(o.into())
            } else {
                None
            },
            controller: if let Some(c) = controller {
                Some(c.into())
            } else {
                None
            },
            project_leader: if let Some(p) = project_leader {
                Some(p.into())
            } else {
                None
            },
            purchased_on: merch.purchased_on,
            count: merch.count,
            cost: merch.cost,
            merchandise_name: merch.merchandise_name,
            use_case: merch.use_case,
            location: merch.location,
            article_number: merch.article_number,
            shop: merch.shop,
            serial_number: merch.serial_number,
            arrived_on: merch.arrived_on,
            url: merch.url,
            postage: merch.postage,
            created_at: merch.created_at,
            updated_at: merch.updated_at,
        }
    }
}
impl From<Model> for InternMerchandise {
    fn from(merch: Model) -> Self {
        Self {
            id: merch.id,
            merchandise_id: None,
            orderer: None,
            controller: None,
            project_leader: None,
            purchased_on: merch.purchased_on,
            count: merch.count,
            cost: merch.cost,
            merchandise_name: merch.merchandise_name,
            use_case: merch.use_case,
            location: merch.location,
            article_number: merch.article_number,
            shop: merch.shop,
            serial_number: merch.serial_number,
            arrived_on: merch.arrived_on,
            url: merch.url,
            postage: merch.postage,
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
    pub project_leader_id: Uuid,
    pub article_number: String,
    pub cost: Decimal,
    pub postage: Decimal,
    pub use_case: Option<String>,
    pub location: Option<String>,
    pub shop: String,
}

#[derive(InputObject, Deserialize, Serialize)]
pub struct IncomingInternMerchandise {
    pub id: Uuid,
    pub merchandise_id: i64,
    pub serial_number: String,
}
