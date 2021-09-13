use async_graphql::{validators::Email, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    models::users::{User as DbUser, UserEmail, UserId},
    validators::Password,
};

#[derive(SimpleObject, Debug, Serialize, Clone)]
pub struct User {
    pub id: UserId,
    pub email: UserEmail,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<DbUser> for User {
    fn from(db_user: DbUser) -> Self {
        Self {
            id: db_user.id,
            email: db_user.email,
            created_at: db_user.created_at,
            firstname: db_user.firstname,
            lastname: db_user.lastname,
            updated_at: db_user.updated_at,
        }
    }
}

#[derive(Deserialize, Debug, InputObject)]
pub struct NewUser {
    #[graphql(validator(Email))]
    pub email: UserEmail,
    #[graphql(validator(Password))]
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(InputObject, Debug, Serialize)]
pub struct PasswordChange {
    pub old_password: String,
    #[graphql(validator(Password))]
    pub new_password: String,
}

#[derive(InputObject, Debug, Serialize)]
pub struct UserUpdate {
    pub firstname: String,
    pub lastname: String,
}
