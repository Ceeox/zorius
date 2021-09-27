use async_graphql::{validators::Email, Enum, InputObject, SimpleObject};
use chrono::{DateTime, FixedOffset};
use sea_orm::Order;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::users::{self, Column, Model, UserEmail},
    validators::Password,
};

#[derive(SimpleObject, Debug, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: UserEmail,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl From<Model> for User {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            email: model.email,
            created_at: model.created_at,
            firstname: model.firstname,
            lastname: model.lastname,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Deserialize, Debug, InputObject)]
pub struct NewUser {
    #[graphql(validator(Email))]
    pub email: UserEmail,
    #[graphql(validator(Password))]
    pub password: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

#[derive(InputObject, Debug, Serialize)]
pub struct PasswordChange {
    pub old_password: String,
    #[graphql(validator(Password))]
    pub new_password: String,
}

#[derive(InputObject, Debug, Serialize)]
pub struct UserUpdate {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

#[derive(Enum, Debug, Serialize, Copy, Clone, PartialEq, Eq)]
pub enum OrderBy {
    Email,
    CreatedAt,
    UpdatedAt,
    Firstname,
    Lastname,
}

impl From<OrderBy> for users::Column {
    fn from(order_by: OrderBy) -> Self {
        match order_by {
            OrderBy::Email => Column::Email,
            OrderBy::CreatedAt => Column::CreatedAt,
            OrderBy::UpdatedAt => Column::UpdatedAt,
            OrderBy::Firstname => Column::Firstname,
            OrderBy::Lastname => Column::Lastname,
        }
    }
}

#[derive(Enum, Debug, Serialize, Copy, Clone, PartialEq, Eq)]
pub enum OrderDirection {
    Asc,
    Desc,
}

impl From<OrderDirection> for Order {
    fn from(order_dic: OrderDirection) -> Self {
        match order_dic {
            OrderDirection::Asc => Order::Asc,
            OrderDirection::Desc => Order::Desc,
        }
    }
}
