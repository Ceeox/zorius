use async_graphql::SimpleObject;
use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::models::user::UserId;

#[derive(Deserialize, Serialize, Debug, SimpleObject)]
pub struct InternMerchandise {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub merchandise_id: Option<i32>,
    //    pub bought_through: Option<CompanyType>,
    pub orderer: UserId,
    pub project_leader: Option<String>,
    pub purchased_on: DateTime,
    pub count: i32,
    pub merchandise_name: String,
    pub use_case: Option<String>,
    pub location: Option<String>,
    pub article_number: Option<String>,
    pub shop: Option<String>,
    pub cost: f64,
    pub serial_number: Option<Vec<String>>,
    pub arived_on: Option<DateTime>,
    //    pub status: InternMerchandiseStatus,
    pub url: Option<String>,
    pub postage: Option<f64>,
    pub invoice_number: Option<i32>,
    pub created_date: DateTime,
    pub updated_date: DateTime,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

/*
#[derive(GraphQLInputObject, Deserialize, Serialize)]
#[graphql(description = "Stores internal merchandise infos")]
pub struct NewInternMerchandiseQuery {
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

#[derive(GraphQLInputObject, Deserialize, Serialize)]
#[graphql(description = "Stores internal merchandise infos")]
pub struct UpdateInternMerchandiseQuery {
    pub merchandise_id: Option<i32>,
    //    pub bought_through: Option<CompanyType>,
    pub orderer: Option<String>,
    pub project_leader: Option<String>,
    pub purchased_on: Option<DateTime>,
    pub count: Option<i32>,
    pub merchandise_name: Option<String>,
    pub use_case: Option<String>,
    pub location: Option<String>,
    pub article_number: Option<String>,
    pub shop: Option<String>,
    pub cost: Option<f64>,
    pub serial_number: Option<Vec<String>>,
    pub arived_on: Option<DateTime>,
    pub status: Option<InternMerchandiseStatus>,
    pub url: Option<String>,
    pub postage: Option<f64>,
    pub invoice_number: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, GraphQLObject)]
pub struct InternMerchandiseResponse {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub merchandise_id: Option<i32>,
    //    pub bought_through: Option<CompanyType>,
    pub orderer: String,
    pub project_leader: Option<String>,
    pub purchased_on: DateTime,
    pub count: i32,
    pub merchandise_name: String,
    pub use_case: Option<String>,
    pub location: Option<String>,
    pub article_number: Option<String>,
    pub shop: Option<String>,
    pub cost: f64,
    pub serial_number: Option<Vec<String>>,
    pub arived_on: Option<DateTime>,
    pub status: InternMerchandiseStatus,
    pub url: Option<String>,
    pub postage: Option<f64>,
    pub invoice_number: Option<i32>,
    pub created_date: DateTime,
    pub updated_date: DateTime,
}

impl InternMerchandise {
    pub fn new(new_intern_merchandise: NewInternMerchandiseQuery) -> Self {
        Self {
            id: ObjectId::new(),
            merchandise_name: new_intern_merchandise.merchandise_name,
            // bought_through: None,
            count: new_intern_merchandise.count,
            orderer: new_intern_merchandise.orderer,
            purchased_on: Utc::now().into(),
            cost: new_intern_merchandise.cost,
            status: InternMerchandiseStatus::Ordered,
            url: new_intern_merchandise.url,
            use_case: new_intern_merchandise.use_case,
            article_number: new_intern_merchandise.article_number,
            postage: new_intern_merchandise.postage,
            project_leader: Some(new_intern_merchandise.project_leader),
            location: Some(new_intern_merchandise.location),
            shop: Some(new_intern_merchandise.shop),

            merchandise_id: None,
            serial_number: None,
            arived_on: None,
            invoice_number: None,
            created_date: Utc::now().into(),
            updated_date: Utc::now().into(),
        }
    }

    // TODO: ugly replace!!!
    pub fn update(&mut self, update: UpdateInternMerchandiseQuery) {
        self.merchandise_id = update.merchandise_id.or(self.merchandise_id);
        self.orderer = update.orderer.unwrap_or(self.orderer.clone());
        self.project_leader = update.project_leader.or(self.project_leader.clone());
        self.purchased_on = update.purchased_on.unwrap_or(self.purchased_on);
        self.count = update.count.unwrap_or(self.count);
        self.merchandise_name = update
            .merchandise_name
            .unwrap_or(self.merchandise_name.clone());
        self.use_case = update.use_case.or(self.use_case.clone());
        self.location = update.location.or(self.location.clone());
        self.article_number = update.article_number.or(self.article_number.clone());
        self.shop = update.shop.or(self.shop.clone());
        self.cost = update.cost.unwrap_or(self.cost);
        self.serial_number = update.serial_number.or(self.serial_number.clone());
        self.arived_on = update.arived_on.or(self.arived_on);
        self.status = update.status.unwrap_or(self.status.clone());
        self.url = update.url.or(self.url.clone());
        self.postage = update.postage.or(self.postage);
        self.invoice_number = update.invoice_number.or(self.invoice_number);
        self.updated_date = Utc::now().into();
    }
}

impl From<InternMerchandise> for InternMerchandiseResponse {
    fn from(intern_merch: InternMerchandise) -> Self {
        Self {
            id: intern_merch.id,
            merchandise_id: intern_merch.merchandise_id,
            orderer: intern_merch.orderer,
            project_leader: intern_merch.project_leader,
            purchased_on: intern_merch.purchased_on,
            count: intern_merch.count,
            merchandise_name: intern_merch.merchandise_name,
            use_case: intern_merch.use_case,
            location: intern_merch.location,
            article_number: intern_merch.article_number,
            shop: intern_merch.shop,
            cost: intern_merch.cost,
            serial_number: intern_merch.serial_number,
            arived_on: intern_merch.arived_on,
            status: intern_merch.status,
            url: intern_merch.url,
            postage: intern_merch.postage,
            invoice_number: intern_merch.invoice_number,
            created_date: intern_merch.created_date,
            updated_date: intern_merch.updated_date,
        }
    }
}
*/
