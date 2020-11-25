use bson::{bson, doc, from_bson, to_bson};
use bson::{de::from_document, oid::ObjectId, to_document, Bson, DateTime};
use chrono::Utc;
use futures::stream::StreamExt;
use juniper::{graphql_value, EmptySubscription, FieldError, FieldResult, RootNode};
use mongodb::options::FindOptions;

use crate::{models::roles::RoleSearch, Context};

static MDB_COLL_NAME_TIME_REC: &str = "time_recordings";

pub struct TimeRecordingQuery;

impl TimeRecordingQuery {
    pub async fn time_record(ctx: &Context, rs: RoleSearch) -> FieldResult<()> {
        unimplemented!()
    }
}

pub struct TimeRecordingMutation;

impl TimeRecordingMutation {
    pub async fn new_time_record(ctx: &Context, user_id: ObjectId) -> FieldResult<()> {
        unimplemented!()
    }

    pub async fn update_time_record(ctx: &Context) -> FieldResult<()> {
        unimplemented!()
    }
}
