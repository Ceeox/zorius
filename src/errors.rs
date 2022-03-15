use std::io::Error as StdIoError;

use actix_web::error::Error as ActixError;
use async_graphql::{Error as GqlError, ErrorExtensions};
use image::error::ImageError;
use jsonwebtoken::errors::Error as JwtError;
use log::error;
use sea_orm::error::DbErr;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error")]
    Io(#[from] StdIoError),
    #[error("token error")]
    Jwt(#[from] JwtError),
    #[error("web server error")]
    Actix(#[from] ActixError),
    #[error("database error")]
    SeaOrm(#[from] DbErr),
    #[error("database error")]
    Image(#[from] ImageError),

    #[error("missing database in context")]
    MissingDatabase,
    #[error("registration is not enabled")]
    RegistrationNotEnabled,
    #[error("email already registered")]
    EmailAlreadyRegistred,
    #[error("incorrect password")]
    IncorrectPassword,
    #[error("not found")]
    NotFound,
    #[error("forbidden")]
    Forbidden,
    #[error("missing token")]
    MissingToken,
    #[error("malformed token")]
    MalformedToken,
    #[error("expired token")]
    ExpiredToken,
    #[error("wrong media type")]
    WrongMediaType,

    #[error("unknown error")]
    Unknown,
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl ErrorExtensions for Error {
    fn extend(&self) -> GqlError {
        GqlError::new(format!("{}", self)).extend_with(|_err, e| match self {
            Error::Io(err) => {
                error!("{err:?}");
                e.set("code", "IO_ERROR")
            }
            Error::Jwt(_) => e.set("code", "JWT_TOKEN_ERROR"),
            Error::Actix(_) => e.set("code", "WEBSERVER_ERROR"),
            Error::SeaOrm(err) => {
                error!("{err:?}");
                e.set("code", "DATABASE_ERROR")
            }

            Error::IncorrectPassword => e.set("code", "INCORRECT_PASSWORD"),
            Error::NotFound => e.set("code", "NOT_FOUND"),
            Error::MalformedToken => e.set("code", "MALFORMED_TOKEN"),
            Error::ExpiredToken => e.set("code", "EXPIRED_TOKEN"),
            Error::MissingToken => e.set("code", "MISSING_TOKEN"),
            Error::EmailAlreadyRegistred => e.set("code", "EMAIL_ALREADY_REGISTERED"),
            Error::RegistrationNotEnabled => e.set("code", "REGISTRATION_NOT_ENABLED"),
            Error::MissingDatabase => e.set("code", "MISSING_DATABASE"),
            Error::Forbidden => e.set("code", "FORBIDDEN"),
            Error::WrongMediaType => e.set("code", "WRONG_MEDIA_TYPE"),
            Error::Image(_) => e.set("code", "IMAGE_ERROR"),

            Error::Unknown => e.set("code", "UNKNOWN"),
        })
    }
}
