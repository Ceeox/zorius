use std::convert::{TryFrom, TryInto};

use async_graphql::{Context, Result};
use chrono::Local;
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::CONFIG, errors::Error};

pub struct Token(pub String);

/// Claim caontains information about the JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    /// Issuer of the JWT
    iss: String,
    /// Subject of the JWT (the user)
    sub: String,
    /// Id of the User (not in JWT standard)
    id: String,
    /// Time after which the JWT expires
    exp: usize,
    /// Time before which the JWT must not be accepted for processing
    nbf: usize,
    /// Time at which the JWT was issued; can be used to determine age of the JWT
    iat: usize,
}

impl Claim {
    /// Creates a new Claim with the users email, id and sets the time when the token expires.
    pub fn new(sub: &str, id: &str, exp: usize) -> Self {
        let mut iss = String::new();
        if CONFIG.web.enable_ssl {
            iss.push_str("https://");
        } else {
            iss.push_str("http://");
        };
        iss.push_str(&CONFIG.domain.clone());
        Self {
            iss,
            sub: sub.to_owned(),
            id: id.to_owned(),
            exp,
            nbf: Local::now().timestamp() as usize,
            iat: Local::now().timestamp() as usize,
        }
    }

    /// Gets the Claim from async_graphql context
    /// `Token(Sting)` must be present in context
    pub fn from_ctx(ctx: &Context<'_>) -> Result<Self, Error> {
        let value: &Token = match ctx.data::<Token>() {
            Err(_e) => return Err(Error::MissingToken),
            Ok(r) => r,
        };
        let claim = Claim::try_from(value.0.to_owned())?;
        if claim.token_expired() {
            return Err(Error::ExpiredToken);
        }
        Ok(claim)
    }

    /// Return a reference to the `user_id`
    pub fn user_id(&self) -> Result<Uuid, Error> {
        match Uuid::parse_str(&self.id) {
            Ok(r) => Ok(r),
            Err(_) => Err(Error::MalformedToken),
        }
    }

    /// Chekcs if the token is expired
    pub fn token_expired(&self) -> bool {
        let now = Local::now().timestamp() as usize;
        self.nbf >= now && self.exp <= now
    }
}

impl TryInto<String> for Claim {
    type Error = jsonwebtoken::errors::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        let key = EncodingKey::from_secret(CONFIG.secret_key.as_bytes());
        let algo = Algorithm::HS512;

        jsonwebtoken::encode(&Header::new(algo), &self, &key)
    }
}

impl TryFrom<String> for Claim {
    type Error = jsonwebtoken::errors::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let secret = CONFIG.secret_key.as_bytes();
        let split = value.split(' ').collect::<Vec<&str>>();
        let token = split.get(1).unwrap_or(&"");

        let dec = DecodingKey::from_secret(secret);
        let vali = Validation::new(Algorithm::HS512);

        Ok(decode::<Claim>(token.trim_matches(' '), &dec, &vali)?.claims)
    }
}
