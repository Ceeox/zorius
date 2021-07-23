use std::fmt::Display;

use askama::Template;
use async_graphql::{validators::IntGreaterThan, Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::CONFIG,
    mailer::mailer,
    models::user::{User, UserId},
    validators::Url,
};

pub type InternMerchandiseId = Uuid;

#[derive(InputObject, Deserialize, Serialize)]
pub struct NewInternMerchandise {
    pub merchandise_name: String,
    #[graphql(validator(IntGreaterThan(value = "0")))]
    pub count: i32,
    #[graphql(validator(Url))]
    pub url: Option<String>,
    pub orderer_id: UserId,
    pub article_number: Option<String>,
    pub cost: f64,
    pub postage: Option<f64>,
    pub use_case: Option<String>,
    pub project_leader_id: UserId,
    pub location: Option<String>,
    pub shop: String,
}

#[derive(Deserialize, Serialize, Debug, SimpleObject, Clone)]
pub struct DBInternMerchandise {
    pub id: InternMerchandiseId,
    pub merchandise_id: Option<i32>,
    pub orderer_id: UserId,
    pub project_leader_id: Option<UserId>,
    pub purchased_on: DateTime<Utc>,
    pub count: i32,
    pub cost: f64,
    pub status: InternMerchandiseStatus,
    pub merchandise_name: String,
    pub use_case: Option<String>,
    pub location: Option<String>,
    pub article_number: Option<String>,
    pub shop: Option<String>,
    pub serial_number: Option<Vec<String>>,
    pub arrived_on: Option<DateTime<Utc>>,
    pub url: Option<String>,
    pub postage: Option<f64>,
    pub invoice_number: Option<i32>,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
}

#[derive(Deserialize, Debug, SimpleObject, Clone)]
pub struct InternMerchandise {
    pub id: InternMerchandiseId,
    pub merchandise_id: Option<i32>,
    pub orderer: User,
    pub project_leader: Option<User>,
    pub purchased_on: DateTime<Utc>,
    pub count: i32,
    pub merchandise_name: String,
    pub use_case: Option<String>,
    pub location: Option<String>,
    pub article_number: Option<String>,
    pub shop: Option<String>,
    pub cost: f64,
    pub serial_number: Option<Vec<String>>,
    pub arrived_on: Option<DateTime<Utc>>,
    pub status: InternMerchandiseStatus,
    pub url: Option<String>,
    pub postage: Option<f64>,
    pub invoice_number: Option<i32>,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
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

impl DBInternMerchandise {
    pub fn new(new_intern_merchandise: NewInternMerchandise) -> Self {
        Self {
            id: InternMerchandiseId::new_v4(),
            merchandise_name: new_intern_merchandise.merchandise_name,
            // bought_through: None,
            count: new_intern_merchandise.count,
            orderer_id: new_intern_merchandise.orderer_id,
            purchased_on: Utc::now().into(),
            cost: new_intern_merchandise.cost,
            status: InternMerchandiseStatus::Ordered,
            url: new_intern_merchandise.url,
            use_case: new_intern_merchandise.use_case,
            article_number: new_intern_merchandise.article_number,
            postage: new_intern_merchandise.postage,
            project_leader_id: Some(new_intern_merchandise.project_leader_id),
            location: new_intern_merchandise.location,
            shop: Some(new_intern_merchandise.shop),

            merchandise_id: None,
            serial_number: None,
            arrived_on: None,
            invoice_number: None,
            created_date: Utc::now().into(),
            updated_date: Utc::now().into(),
        }
    }

    pub fn change_status(&mut self, new_status: InternMerchandiseStatus, user: User) {
        self.status = new_status;
        self.updated_date = Utc::now().into();
        let orderer_name = format!("{} {}", user.firstname, user.lastname);
        let template: StatusTemplate = StatusTemplate {
            id: self.id.clone(),
            merchandise_id: self.merchandise_id,
            orderer_name,
            count: self.count,
            merchandise_name: self.merchandise_name.clone(),
            cost: self.cost,
            status: new_status,
        };
        let body = template.render().unwrap();

        mailer(
            &CONFIG.mailer.merchandise_email_send_to,
            &format!(
                "Intern Merchandise Staus Change to {} for {}",
                new_status.to_string(),
                self.merchandise_name,
            ),
            &body,
        );
    }
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
    pub cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrived_on: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<i32>,
}

#[derive(Template)]
#[template(path = "intern_merch_used.html")]
pub struct StatusTemplate {
    pub(crate) id: InternMerchandiseId,
    pub(crate) merchandise_id: Option<i32>,
    pub(crate) orderer_name: String,
    pub(crate) count: i32,
    pub(crate) merchandise_name: String,
    pub(crate) cost: f64,
    pub(crate) status: InternMerchandiseStatus,
}
