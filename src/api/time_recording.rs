use std::time::Duration;

use bson::{doc, from_document, to_document, DateTime};
use futures::{future, stream::StreamExt};
use juniper::{graphql_value, FieldError, FieldResult};
use mongodb::Cursor;

use crate::{
    models::time_recording::time_record::TimeRecord,
    models::{
        time_recording::time_record::{TimeRecordId, UpdateTimeRecord},
        user::UserId,
    },
    Context,
};

static MDB_COLL_NAME_TIME_REC: &str = "time_recordings";

pub struct TimeRecordingQuery;

impl TimeRecordingQuery {
    pub async fn list_time_records(ctx: &Context, user_id: UserId) -> FieldResult<Vec<TimeRecord>> {
        let collection = ctx.db.collection(MDB_COLL_NAME_TIME_REC);
        let filter = doc! {
            "user_id": user_id,
        };
        let cursor: Cursor = collection.find(filter, None).await?;
        let res = cursor
            .filter(|doc| future::ready(doc.is_ok()))
            .map(|doc| from_document::<TimeRecord>(doc.unwrap()).unwrap())
            .collect::<Vec<_>>()
            .await;
        Ok(res)
    }

    pub async fn started(ctx: &Context, tr_id: TimeRecordId) -> FieldResult<DateTime> {
        let tr = get_single_tr(ctx, tr_id).await?;
        Ok(tr.started())
    }

    pub async fn ended(ctx: &Context, tr_id: TimeRecordId) -> FieldResult<Option<DateTime>> {
        let tr = get_single_tr(ctx, tr_id).await?;
        Ok(tr.ended())
    }

    pub async fn duration(ctx: &Context, tr_id: TimeRecordId) -> FieldResult<Option<Duration>> {
        let tr = get_single_tr(ctx, tr_id).await?;
        Ok(tr.get_duration())
    }
}

pub struct TimeRecordingMutation;

impl TimeRecordingMutation {
    pub async fn new_time_record(ctx: &Context, user_id: UserId) -> FieldResult<TimeRecord> {
        let tr = TimeRecord::new(user_id);
        let collection = ctx.db.collection(MDB_COLL_NAME_TIME_REC);
        let doc = to_document(&tr)?;
        let _ = collection.insert_one(doc.clone(), None).await?;
        Ok(tr.into())
    }

    pub async fn update_time_record(ctx: &Context, tr_id: TimeRecordId) -> FieldResult<TimeRecord> {
        todo!()
    }

    pub async fn end_time_record(ctx: &Context, tr_id: TimeRecordId) -> FieldResult<TimeRecord> {
        let mut tr = get_single_tr(ctx, tr_id).await?;
        tr.end();
        // TODO: impl set_single_tr
        todo!();
        //Ok(tr)
    }
}

// async fn set_single_tr(ctx: &Context, tr_update: UpdateTimeRecord) -> FieldResult<TimeRecord> {
//     let filter = doc! { "_id": tr_update.id };
//     let collection = ctx.db.collection(MDB_COLL_NAME_TIME_REC);
//     let tr_doc = to_document(&tr_update)?;
//     let _ = collection.update_one(filter, tr_doc, None).await?;
//     Ok(tr_update.into())
// }

async fn get_single_tr(ctx: &Context, tr_id: TimeRecordId) -> FieldResult<TimeRecord> {
    let collection = ctx.db.collection(MDB_COLL_NAME_TIME_REC);
    let filter = doc! {
        "id": tr_id,
    };
    match collection.find_one(filter, None).await? {
        None => {
            return Err(FieldError::new(
                "specified time record not found",
                graphql_value!({ "error": "specified time record not found" }),
            ));
        }
        Some(r) => Ok(from_document(r)?),
    }
}
