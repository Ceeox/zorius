use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    guard::Guard,
    Context, Error, Object, Result,
};
use bson::{doc, from_document, to_document};
use futures::StreamExt;
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};

use crate::{
    api::{claim::Claim, database, MDB_COLL_NAME_USERS, MDB_COLL_WORK_REPORTS},
    models::{
        roles::{Role, RoleGuard},
        work_report::{
            NewWorkReport, WorkReport, WorkReportId, WorkReportResponse, WorkReportUpdate,
        },
    },
};

use super::database2;

#[derive(Default)]
pub struct WorkReportQuery;

#[Object]
impl WorkReportQuery {
    async fn get_workreport_by_id(
        &self,
        ctx: &Context<'_>,
        id: WorkReportId,
    ) -> Result<Option<WorkReportResponse>> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();

        match database2(ctx)?
            .get_work_report_by_id(id, user_id.clone())
            .await?
        {
            Some(r) => Ok(Some(r)),
            None => Err(Error::new("work report not found!")),
        }
    }

    async fn list_work_reports(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, WorkReportResponse, EmptyFields, EmptyFields>> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();
        let collection = database(ctx)?.collection(MDB_COLL_WORK_REPORTS);
        let doc_count = collection.estimated_document_count(None).await? as usize;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = after.unwrap_or(0);
                let mut end = before.unwrap_or(doc_count);

                if let Some(first) = first {
                    end = (start + first).min(end);
                }
                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last };
                }
                let limit = (end - start) as i64;
                let pipeline = vec![
                    doc! {"$skip": start as i64},
                    doc! {"$limit": limit },
                    doc! {"$match": {
                            "user_id": user_id
                    }},
                    doc! {"$lookup": {
                            "from": MDB_COLL_NAME_USERS,
                            "localField": "user_id",
                            "foreignField": "_id",
                            "as": "user"
                        }
                    },
                    doc! {"$lookup": {
                            "from": MDB_COLL_WORK_REPORTS,
                            "localField": "project_id",
                            "foreignField": "_id",
                            "as": "project"
                        }
                    },
                    doc! {"$unwind": {
                            "path": "$user_id",
                        }
                    },
                    doc! {"$unwind": {
                            "path": "$project_id"
                        }
                    },
                ];
                let cursor = collection.aggregate(pipeline, None).await?;

                let mut connection = Connection::new(start > 0, end < doc_count);
                connection
                    .append_stream(cursor.enumerate().map(|(n, doc)| {
                        let merch = from_document::<WorkReportResponse>(doc.unwrap()).unwrap();
                        Edge::with_additional_fields(n + start, merch, EmptyFields)
                    }))
                    .await;
                Ok(connection)
            },
        )
        .await
    }
}

#[derive(Default)]
pub struct WorkReportMutation;

#[Object]
impl WorkReportMutation {
    async fn new_work_report(&self, ctx: &Context<'_>, new: NewWorkReport) -> Result<WorkReport> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();

        let collection = database(ctx)?.collection(MDB_COLL_WORK_REPORTS);
        let wr = WorkReport::new(user_id.clone(), new);
        let insert = to_document(&wr)?;
        let _ = collection.insert_one(insert, None).await?;
        Ok(wr)
    }

    async fn update_work_report(
        &self,
        ctx: &Context<'_>,
        id: WorkReportId,
        update: WorkReportUpdate,
    ) -> Result<WorkReportResponse> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();
        Ok(database2(ctx)?
            .update_work_report(id, user_id.clone(), update)
            .await?)
    }
}
