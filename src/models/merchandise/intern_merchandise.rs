use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::company::CompanyType;

#[derive(Deserialize, Serialize, Debug, juniper::GraphQLInputObject)]
pub struct InternMerchandise {
    #[serde(rename = "_id")]
    pub id: String,

    pub merchandise_id: Option<i32>,
    pub bought_through: Option<CompanyType>,
    pub orderer: String,
    pub project_leader: Option<String>,
    pub purchased_on: DateTime<Utc>,

    pub count: i32,
    pub merchandise_name: String,
    pub use_case: Option<String>,
    pub location: Option<String>,
    pub article_number: Option<String>,
    pub shop: Option<String>,
    pub cost: f64,

    pub serial_number: Option<Vec<String>>,
    pub arived_on: Option<String>,
    pub status: InternMerchandiseStatus,
    pub url: Option<String>,
    pub postage: Option<f64>,
    pub invoice_number: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, juniper::GraphQLInputObject)]
pub struct InternMerchandiseList {
    pub intern_list: Vec<InternMerchandise>,
}

#[derive(juniper::GraphQLEnum, Deserialize, Serialize, Debug)]
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

#[derive(juniper::GraphQLInputObject, Deserialize, Serialize)]
#[graphql(description = "Stores internal merchandise infos")]
pub struct NewInternOrder {
    pub merchandise_name: String,
    pub count: i32,
    pub url: Option<String>,
    pub orderer: String,
    pub article_number: Option<String>,
    pub cost: f64,
    pub postage: Option<f64>,
    pub use_case: Option<String>,
    pub bought_through: Option<CompanyType>,
    pub project_leader: String,
    pub location: String,
    pub shop: String,
}
