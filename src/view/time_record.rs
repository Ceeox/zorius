use async_graphql::{Enum, InputObject, SimpleObject};
use entity::{customer, project, time_record::*, user};
use sea_orm::prelude::DateTimeUtc;
use serde::Serialize;
use uuid::Uuid;

use crate::view::{customer::Customer, project::Project, users::User};

#[derive(SimpleObject, Debug, Serialize, Clone)]
pub struct TimeRecord {
    pub id: i32,
    pub start: DateTimeUtc,
    pub end: Option<DateTimeUtc>,
}

impl From<Model> for TimeRecord {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            start: model.start,
            end: model.end,
        }
    }
}
