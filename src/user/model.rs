use async_graphql::{InputObject, Object, SimpleObject};
use entity::user::Model;
use pwhash::sha512_crypt;
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::MutationType;
use crate::validators::Password;

#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, SimpleObject)]
pub struct LoginResult {
    pub token: String,
}

#[derive(SimpleObject, Debug, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[graphql(secret = true, visible = false)]
    pub password_hash: String,
    pub name: Option<String>,
    pub avatar_filename: Option<String>,
    pub is_admin: bool,
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
            is_admin: model.is_admin,
            avatar_filename: model.avatar_filename,
            created_at: model.created_at,
            updated_at: model.updated_at,
            deleted_at: model.deleted_at,
        }
    }
}

#[derive(Deserialize, Debug, InputObject, SimpleObject, Default)]
pub struct ListUserOptions {
    pub ids: Option<Vec<Uuid>>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub first: Option<i32>,
    pub last: Option<i32>,
}

#[derive(Debug, Default)]
pub struct DbListOptions {
    pub ids: Option<Vec<Uuid>>,
    pub start: u64,
    pub limit: u64,
}

#[derive(Deserialize, Debug, InputObject, SimpleObject)]
pub struct NewUser {
    #[graphql(validator(email))]
    pub email: String,
    #[graphql(validator(custom = "Password"))]
    pub password: String,
    pub name: Option<String>,
    #[graphql(visible = false)]
    pub is_admin: Option<bool>,
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
    pub is_admin: Option<bool>,
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
