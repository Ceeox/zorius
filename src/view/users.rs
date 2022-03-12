use async_graphql::{Enum, InputObject, Object, SimpleObject};
use entity::user::Model;
use entity::user::{self, Column};
use pwhash::sha512_crypt;
use sea_orm::{prelude::DateTimeUtc, Order};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::MutationType;
use crate::validators::Password;

#[derive(SimpleObject, Debug, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip)]
    #[graphql(visible = false)]
    pub password_hash: String,
    pub name: Option<String>,
    pub avatar_filename: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,
}

impl User {
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn hash_password(password: &str) -> String {
        sha512_crypt::hash(password.as_bytes())
            .expect("system random number generator cannot be opened!")
    }

    pub fn is_password_correct(&self, password: &str) -> bool {
        sha512_crypt::verify(password.as_bytes(), &self.password_hash)
    }
}

impl From<Model> for User {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            email: model.email,
            password_hash: model.password_hash,
            name: model.name,
            avatar_filename: model.avatar_filename,
            created_at: model.created_at,
            updated_at: model.updated_at,
            deleted_at: model.deleted_at,
        }
    }
}

#[derive(Deserialize, Debug, InputObject)]
pub struct NewUser {
    #[graphql(validator(email))]
    pub email: String,
    #[graphql(validator(custom = "Password"))]
    pub password: String,
    pub name: Option<String>,
}

#[derive(InputObject, Debug, Serialize)]
pub struct PasswordChange {
    pub old_password: String,
    #[graphql(validator(custom = "Password"))]
    pub new_password: String,
}

#[derive(InputObject, Debug, Serialize)]
pub struct UserUpdate {
    pub name: Option<String>,
}

#[derive(Enum, Debug, Serialize, Copy, Clone, PartialEq, Eq)]
pub enum OrderBy {
    Email,
    CreatedAt,
    UpdatedAt,
    Name,
}

impl From<OrderBy> for user::Column {
    fn from(order_by: OrderBy) -> Self {
        match order_by {
            OrderBy::Email => Column::Email,
            OrderBy::CreatedAt => Column::CreatedAt,
            OrderBy::UpdatedAt => Column::UpdatedAt,
            OrderBy::Name => Column::Name,
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

#[derive(Clone)]
pub struct UserChanged {
    pub mutation_type: MutationType,
    pub id: Uuid,
}

#[Object]
impl UserChanged {
    async fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }

    async fn id(&self) -> &Uuid {
        &self.id
    }
}
