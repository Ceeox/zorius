use actix_web::web::HttpResponse;
use actix_web::{dev::Body, error::Error as ActixError, http::StatusCode, ResponseError};
use bson::{de::Error as DeError, ser::Error as SerError};
use jsonwebtoken::errors::Error as JWTError;
use juniper::FieldError;
use mongodb::error::Error as MngDBError;

use std::fmt;
use std::io::Error as StdIoError;

#[derive(Debug)]
pub enum ZoriusError {
    MongoDBError(MngDBError),
    IoError(StdIoError),
    JuniperError(FieldError),
    BsonDecodeError(DeError),
    BsonEncodeError(SerError),
    JWTError(JWTError),
    AuthError(ZoriusAuthError),
    ActixError(ActixError),
}

#[derive(Debug)]
pub enum ZoriusAuthError {
    WrongUserOrPassword,
}

impl fmt::Display for ZoriusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZoriusError::MongoDBError(e) => f.write_str(&format!("{}", e)),
            ZoriusError::IoError(e) => f.write_str(&format!("{}", e)),
            ZoriusError::JuniperError(e) => f.write_str(&format!("JuniperError: {:?}", e)),
            ZoriusError::BsonDecodeError(e) => f.write_str(&format!("{}", e)),
            ZoriusError::BsonEncodeError(e) => f.write_str(&format!("{}", e)),
            ZoriusError::JWTError(e) => f.write_str(&format!("{}", e)),
            ZoriusError::AuthError(e) => f.write_str(&format!("{:?}", e)),
            ZoriusError::ActixError(e) => f.write_str(&format!("{:?}", e)),
        }
    }
}

impl ResponseError for ZoriusError {
    fn status_code(&self) -> StatusCode {
        match self {
            ZoriusError::MongoDBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ZoriusError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ZoriusError::JuniperError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ZoriusError::BsonDecodeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ZoriusError::BsonEncodeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ZoriusError::JWTError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ZoriusError::AuthError(_) => StatusCode::NOT_FOUND,
            ZoriusError::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<Body> {
        match self {
            ZoriusError::MongoDBError(_) => HttpResponse::InternalServerError().finish(),
            ZoriusError::IoError(_) => HttpResponse::InternalServerError().finish(),
            ZoriusError::JuniperError(_) => HttpResponse::InternalServerError().finish(),
            ZoriusError::BsonDecodeError(_) => HttpResponse::InternalServerError().finish(),
            ZoriusError::BsonEncodeError(_) => HttpResponse::InternalServerError().finish(),
            ZoriusError::JWTError(_) => HttpResponse::InternalServerError().finish(),
            ZoriusError::AuthError(_) => HttpResponse::NotFound().finish(),
            ZoriusError::ActixError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

impl From<ActixError> for ZoriusError {
    fn from(err: ActixError) -> Self {
        ZoriusError::ActixError(err)
    }
}

impl From<StdIoError> for ZoriusError {
    fn from(err: StdIoError) -> Self {
        ZoriusError::IoError(err)
    }
}

impl From<MngDBError> for ZoriusError {
    fn from(err: MngDBError) -> Self {
        ZoriusError::MongoDBError(err)
    }
}

impl From<FieldError> for ZoriusError {
    fn from(err: FieldError) -> Self {
        ZoriusError::JuniperError(err)
    }
}

impl From<DeError> for ZoriusError {
    fn from(err: DeError) -> Self {
        ZoriusError::BsonDecodeError(err)
    }
}

impl From<SerError> for ZoriusError {
    fn from(err: SerError) -> Self {
        ZoriusError::BsonEncodeError(err)
    }
}

impl From<JWTError> for ZoriusError {
    fn from(err: JWTError) -> Self {
        ZoriusError::JWTError(err)
    }
}
