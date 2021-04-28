use std::convert::TryFrom;

use async_graphql::{Context, Error, Result};
use chrono::Local;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::{config::CONFIG, models::user::UserId};

pub struct Token(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    iss: String,
    sub: String,
    id: UserId,
    exp: usize,
    nbf: usize,
    iat: usize,
}

impl Claim {
    /// Creates a new Claim with the users email, id and sets the time when the token expires.
    pub fn new(sub: String, id: UserId, exp: usize) -> Self {
        let mut iss = String::new();
        if CONFIG.web.enable_ssl {
            iss.push_str("https://");
        } else {
            iss.push_str("http://");
        };
        iss.push_str(&CONFIG.domain.clone());
        Self {
            iss,
            sub,
            id,
            exp,
            nbf: Local::now().timestamp() as usize,
            iat: Local::now().timestamp() as usize,
        }
    }

    /// Get the Claim from async_graphql context
    /// `Token(Sting)` must be present in Context
    pub fn from_ctx(ctx: &Context<'_>) -> Result<Self> {
        let value: &Token = match ctx.data::<Token>() {
            Err(_e) => return Err(Error::new("missing token")),
            Ok(r) => r,
        };
        Ok(Claim::try_from(value.0.clone())?)
    }

    /// return a reference to the `user_id`
    pub fn user_id(&self) -> &UserId {
        &self.id
    }

    /// retruns the unix timestamp when the token expries.
    pub fn expires_at(&self) -> usize {
        self.exp
    }
}

impl TryFrom<String> for Claim {
    type Error = async_graphql::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let _split: Vec<&str> = value.split("Bearer").collect();
        let token = match _split.get(1) {
            Some(token) => token.trim(),
            None => return Err(Error::new("missing token")),
        };

        let key = CONFIG.secret_key.as_bytes();
        match decode::<Claim>(
            token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(data) => Ok(data.claims),
            Err(e) => Err(Error::new(&e.to_string())),
        }
    }
}
