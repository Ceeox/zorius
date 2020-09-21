use bson::{de::Error as DeError, ser::Error as SerError};
use juniper::FieldError;
use mongodb::error::Error as MngDBError;
use std::io::Error as StdIoError;

#[derive(Debug)]
pub enum ZoriusError {
    MongoDBError(MngDBError),
    IoError(StdIoError),
    JuniperError(FieldError),
    BsonDecodeError(DeError),
    BsonEncodeError(SerError),
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
