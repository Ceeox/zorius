use crate::config::Config;
use crate::models::user::Claim;

use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::config::CONFIG;
pub struct AuthorizationService;

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;
    type Config = Config;

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let auth = _req.headers().get("Authorization");
        match auth {
            Some(_) => {
                let _split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();
                let key = CONFIG.secret_key.as_bytes();
                match decode::<Claim>(
                    token,
                    &DecodingKey::from_secret(key),
                    &Validation::new(Algorithm::HS512),
                ) {
                    Ok(_token) => ok(AuthorizationService),
                    Err(_e) => err(ErrorUnauthorized("invalid token!")),
                }
            }
            None => err(ErrorUnauthorized("blocked!")),
        }
    }
}
