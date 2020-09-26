use bson::{bson, doc, from_bson, to_bson};
use bson::{de::from_document, oid::ObjectId, to_document, Bson, DateTime};
use chrono::Utc;
use futures::stream::StreamExt;
use juniper::{graphql_value, EmptySubscription, FieldError, FieldResult, RootNode};
use mongodb::options::FindOptions;

use crate::models::merchandise::intern_merchandise::{
    InternMerchandise, InternMerchandiseList, InternMerchandiseStatus, InternMerchandiseUpdate,
    NewInternOrder,
};

use crate::Context;

static MONGO_DB_COLLECTION_NAME: &str = "time_recordings";

pub struct TimeRecordingQuery;

impl TimeRecordingQuery {}

pub struct TimeRecordingMutation;

impl TimeRecordingMutation {}
