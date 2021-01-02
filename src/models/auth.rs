use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, SimpleObject)]
pub struct LoginResult {
    pub token: String,
    pub expires_at: usize,
}
