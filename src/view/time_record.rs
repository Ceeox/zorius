use async_graphql::SimpleObject;
use entity::time_record::*;
use sea_orm::prelude::DateTimeUtc;
use serde::Serialize;

#[derive(SimpleObject, Debug, Serialize, Clone)]
pub struct TimeRecord {
    pub start: DateTimeUtc,
    pub end: Option<DateTimeUtc>,
}

impl From<Model> for TimeRecord {
    fn from(model: Model) -> Self {
        Self {
            start: model.start,
            end: model.end,
        }
    }
}
