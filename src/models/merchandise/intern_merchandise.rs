use std::{convert::TryFrom, fmt::Display};

use async_graphql::{validators::IntGreaterThan, Enum, InputObject, SimpleObject};
use bson::{oid::ObjectId, DateTime, Document};
use chrono::Utc;
use mongod::{AsFilter, AsUpdate, Collection, Comparator, Filter, Update};
use serde::{Deserialize, Serialize};

use crate::{helper::validators::Url, models::user::UserId};

#[derive(InputObject, Deserialize, Serialize)]
pub struct NewMerchandiseIntern {
    pub merchandise_name: String,
    #[graphql(validator(IntGreaterThan(value = "1")))]
    pub count: i32,
    #[graphql(validator(Url))]
    pub url: Option<String>,
    pub orderer: UserId,
    pub article_number: Option<String>,
    pub cost: f64,
    pub postage: Option<f64>,
    pub use_case: Option<String>,
    pub project_leader: String,
    pub location: String,
    pub shop: String,
}

#[derive(Deserialize, Serialize, Debug, SimpleObject, Clone)]
pub struct InternMerchandise {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub merchandise_id: Option<i32>,
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
    pub status: InternMerchandiseStatus,
    pub url: Option<String>,
    pub postage: Option<f64>,
    pub invoice_number: Option<i32>,
    pub created_date: DateTime,
    pub updated_date: DateTime,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Enum)]
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

impl InternMerchandise {
    pub fn new(new_intern_merchandise: NewMerchandiseIntern) -> Self {
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
}

impl Collection for InternMerchandise {
    const COLLECTION: &'static str = "intern_merchandise";

    fn from_document(document: Document) -> Result<Self, mongod::Error> {
        match bson::from_document::<Self>(document) {
            Ok(user) => Ok(user),
            Err(_) => Err(mongod::Error::invalid_document("missing required fields")),
        }
    }

    fn into_document(self) -> Result<Document, mongod::Error> {
        match bson::to_document::<Self>(&self) {
            Ok(doc) => Ok(doc),
            Err(_) => Err(mongod::Error::invalid_document("missing required fields")),
        }
    }
}

#[derive(Default)]
pub struct InternMerchandiseFilter {
    pub id: Option<Comparator<ObjectId>>,
}

impl Filter for InternMerchandiseFilter {
    fn new() -> Self {
        Self::default()
    }

    fn into_document(self) -> Result<Document, mongod::Error> {
        let mut doc = Document::new();
        if let Some(value) = self.id {
            doc.insert("_id", mongod::ext::bson::Bson::try_from(value)?.0);
        }
        Ok(doc)
    }
}

impl AsFilter<InternMerchandiseFilter> for ObjectId {
    fn filter() -> InternMerchandiseFilter {
        InternMerchandiseFilter::default()
    }

    fn into_filter(self) -> InternMerchandiseFilter {
        InternMerchandiseFilter {
            id: Some(Comparator::Eq(self)),
        }
    }
}

impl AsFilter<InternMerchandiseFilter> for InternMerchandise {
    fn filter() -> InternMerchandiseFilter {
        InternMerchandiseFilter::default()
    }

    fn into_filter(self) -> InternMerchandiseFilter {
        InternMerchandiseFilter {
            id: Some(Comparator::Eq(self.id)),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, InputObject, Default)]
pub struct InternMerchandiseUpdate {
    pub merchandise_id: Option<i32>,
    pub orderer: Option<UserId>,
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
    pub created_date: Option<DateTime>,
}

impl Update for InternMerchandiseUpdate {
    fn new() -> Self {
        Self::default()
    }

    fn into_document(self) -> Result<Document, mongod::Error> {
        let mut doc = Document::new();
        if let Some(value) = self.merchandise_id {
            doc.insert("merchandise_id", value);
        }
        if let Some(value) = self.orderer {
            doc.insert("orderer", value);
        }
        if let Some(value) = self.project_leader {
            doc.insert("project_leader", value);
        }
        if let Some(value) = self.count {
            doc.insert("count", value);
        }
        if let Some(value) = self.merchandise_name {
            doc.insert("merchandise_name", value);
        }
        if let Some(value) = self.use_case {
            doc.insert("use_case", value);
        }
        if let Some(value) = self.location {
            doc.insert("location", value);
        }
        if let Some(value) = self.article_number {
            doc.insert("article_number", value);
        }
        if let Some(value) = self.shop {
            doc.insert("shop", value);
        }
        if let Some(value) = self.cost {
            doc.insert("cost", value);
        }
        if let Some(value) = self.serial_number {
            doc.insert("serial_number", value);
        }
        if let Some(value) = self.arived_on {
            doc.insert("arived_on", bson::Bson::DateTime(value.into()));
        }
        if let Some(value) = self.status {
            doc.insert("status", value.to_string());
        }
        if let Some(value) = self.url {
            doc.insert("url", value);
        }
        if let Some(value) = self.postage {
            doc.insert("postage", value);
        }
        if let Some(value) = self.invoice_number {
            doc.insert("invoice_number", value);
        }
        if let Some(value) = self.created_date {
            doc.insert("created_date", bson::Bson::DateTime(value.into()));
        }
        doc.insert("updated_date", Utc::now());
        Ok(doc)
    }
}

impl AsUpdate<InternMerchandiseUpdate> for InternMerchandiseUpdate {
    fn update() -> InternMerchandiseUpdate {
        InternMerchandiseUpdate::default()
    }

    fn into_update(self) -> InternMerchandiseUpdate {
        self
    }
}

impl AsUpdate<InternMerchandiseUpdate> for InternMerchandise {
    fn update() -> InternMerchandiseUpdate {
        InternMerchandiseUpdate::default()
    }

    fn into_update(self) -> InternMerchandiseUpdate {
        InternMerchandiseUpdate {
            merchandise_id: self.merchandise_id,
            orderer: Some(self.orderer),
            project_leader: self.project_leader,
            purchased_on: Some(self.purchased_on),
            count: Some(self.count),
            merchandise_name: Some(self.merchandise_name),
            use_case: self.use_case,
            location: self.location,
            article_number: self.article_number,
            shop: self.shop,
            cost: Some(self.cost),
            serial_number: self.serial_number,
            arived_on: self.arived_on,
            status: Some(self.status),
            url: self.url,
            postage: self.postage,
            invoice_number: self.invoice_number,
            created_date: Some(self.created_date),
        }
    }
}
