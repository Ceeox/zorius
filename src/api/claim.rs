use std::convert::TryFrom;

use async_graphql::{Context, Error, Result};
use chrono::Local;
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{config::CONFIG, models::user::UserId};

pub struct Token(pub String);

static ALGO: Algorithm = Algorithm::HS512;

/// Claim caontains information about the JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    /// Issuer of the JWT
    iss: String,
    /// Subject of the JWT (the user)
    sub: String,
    /// Id of the User (not in JWT standard)
    id: UserId,
    /// Time after which the JWT expires
    exp: usize,
    /// Time before which the JWT must not be accepted for processing
    nbf: usize,
    /// Time at which the JWT was issued; can be used to determine age of the JWT
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

    /// Gets the Claim from async_graphql context
    /// `Token(Sting)` must be present in context
    pub fn from_ctx(ctx: &Context<'_>) -> Result<Self> {
        let value: &Token = match ctx.data::<Token>() {
            Err(_e) => return Err(Error::new("missing token")),
            Ok(r) => r,
        };
        let claim = Claim::try_from(value.0.to_owned())?;
        if claim.token_expired() {
            return Err(Error::new("Token expired!"));
        }
        Ok(claim)
    }

    /// Return a reference to the `user_id`
    pub fn user_id(&self) -> &UserId {
        &self.id
    }

    /// Retruns the unix timestamp when the token expries.
    pub fn expires_at(&self) -> usize {
        self.exp
    }

    /// Retruns if the token is expired
    pub fn token_expired(&self) -> bool {
        let now = Local::now().timestamp() as usize;
        self.nbf >= now && self.exp <= now
    }
}

impl ToString for Claim {
    fn to_string(&self) -> String {
        let key = &EncodingKey::from_secret(&CONFIG.secret_key.as_bytes());
        jsonwebtoken::encode(&Header::new(ALGO), self, key).expect("failed jwt convert to string")
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
            &Validation::new(ALGO),
        ) {
            Ok(data) => Ok(data.claims),
            Err(e) => Err(Error::new(&e.to_string())),
        }
    }
}
