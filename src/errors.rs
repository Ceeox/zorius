use actix_web::{
    error::Error as ActixError, http::StatusCode, HttpResponse, Responder, ResponseError,
};
use jsonwebtoken::errors::Error as JWTError;
use sqlx::Error as SqlxError;

use std::fmt;
use std::io::Error as StdIoError;

#[derive(Debug)]
pub enum ZoriusError {
    IoError(StdIoError),
    JWTError(JWTError),
    ActixError(ActixError),
    SqlxError(SqlxError),
}

impl fmt::Display for ZoriusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZoriusError::IoError(e) => f.write_str(&format!("{}", e)),
            ZoriusError::JWTError(e) => f.write_str(&format!("{}", e)),
            ZoriusError::ActixError(e) => f.write_str(&format!("{:?}", e)),
            ZoriusError::SqlxError(e) => f.write_str(&format!("{:?}", e)),
        }
    }
}

impl ResponseError for ZoriusError {
    fn status_code(&self) -> StatusCode {
        match self {
            ZoriusError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ZoriusError::JWTError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ZoriusError::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ZoriusError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            ZoriusError::IoError(_)
            | ZoriusError::JWTError(_)
            | ZoriusError::ActixError(_)
            | ZoriusError::SqlxError(_) => HttpResponse::InternalServerError().finish(),
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

impl From<JWTError> for ZoriusError {
    fn from(err: JWTError) -> Self {
        ZoriusError::JWTError(err)
    }
}
