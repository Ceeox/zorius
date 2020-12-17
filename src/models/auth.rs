use serde::{Deserialize, Serialize};

use super::user::NewUserQuery;

#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResult {
    pub token: String,
}

pub type Register = NewUserQuery;
