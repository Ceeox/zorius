use chrono::Duration;

use actix_web::{
    cookie::Cookie,
    web::{self},
    HttpResponse,
};
use bson::{doc, from_document, to_document};
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header};

use crate::{
    api::user::MDB_COLL_NAME_USERS,
    config::CONFIG,
    errors::ZoriusError,
    models::{
        auth::{LoginData, LoginResult, Register},
        user::{Claim, User, UserResponse},
    },
    Context,
};

/// Logs the Provided user with an email an password in.
/// It returns a token and sets a cookie
pub async fn login(
    ctx: web::Data<Context>,
    data: web::Json<LoginData>,
) -> Result<HttpResponse, ZoriusError> {
    let collection = ctx.db.collection(MDB_COLL_NAME_USERS);
    let filter = doc! { "email": data.email.to_owned() };
    let user: User = match collection.find_one(filter, None).await? {
        None => {
            return Ok(HttpResponse::NotFound().finish());
        }
        Some(r) => from_document(r).unwrap(),
    };

    if !user.is_password_correct(&data.password) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let claims = Claim {
        sub: user.email().to_owned(),
        exp: (Utc::now() + Duration::days(30)).timestamp() as usize,
    };
    let key = &EncodingKey::from_secret(&CONFIG.secret_key.as_bytes());
    let token = jsonwebtoken::encode(&Header::new(Algorithm::HS512), &claims, key)?;

    let res = LoginResult {
        token: token.clone(),
    };
    let mut cookie = Cookie::new("token", &token);
    cookie.set_path("/api/v1/graphql");

    Ok(HttpResponse::Ok().cookie(cookie).json(res))
}

// TODO: implement email verification!
// TODO: check if e-mail is already used!
pub async fn register(
    ctx: web::Data<Context>,
    data: web::Json<Register>,
) -> Result<HttpResponse, ZoriusError> {
    let user = User::new(data.0);
    let collection = ctx.db.collection(MDB_COLL_NAME_USERS);
    let doc = to_document(&user)?;
    let _ = collection.insert_one(doc.clone(), None).await?;
    Ok(HttpResponse::Ok().json::<UserResponse>(user.into()))
}
