use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    guard::Guard,
    Context, Object, Result,
};
use futures::StreamExt;

use crate::{
    api::{claim::Claim, database},
    models::{
        roles::{Role, RoleGuard},
        work_report::{NewWorkReport, WorkReport, WorkReportId, WorkReportUpdate},
    },
};

#[derive(Default)]
pub struct WorkReportQuery;

#[Object]
impl WorkReportQuery {
    async fn get_workreport_by_id(
        &self,
        ctx: &Context<'_>,
        id: WorkReportId,
    ) -> Result<WorkReport> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();

        Ok(database(ctx)?
            .get_work_report_by_id(id, user_id.clone())
            .await?)
    }

    async fn list_work_reports(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, WorkReport, EmptyFields, EmptyFields>> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();
        let doc_count = database(ctx)?.count_work_reports().await?;

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

                let cursor = database(ctx)?
                    .list_work_report(user_id.clone(), start as i64, limit)
                    .await?;

                let mut connection = Connection::new(start > 0, end < doc_count);
                connection
                    .append_stream(cursor.enumerate().map(|(n, doc)| {
                        let wr = from_document::<WorkReport>(doc.unwrap()).unwrap();
                        Edge::with_additional_fields(n + start, wr, EmptyFields)
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

        Ok(database(ctx)?.new_work_report(user_id.clone(), new).await?)
    }

    async fn update_work_report(
        &self,
        ctx: &Context<'_>,
        id: WorkReportId,
        update: WorkReportUpdate,
    ) -> Result<WorkReport> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();
        let _ = database(ctx)?
            .update_work_report(id.clone(), user_id.clone(), update)
            .await?;
        Ok(database(ctx)?
            .get_work_report_by_id(id, user_id.clone())
            .await?)
    }

    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::MerchandiseModerator")
    )))]
    async fn delete_work_report(&self, ctx: &Context<'_>, id: WorkReportId) -> Result<bool> {
        let _ = Claim::from_ctx(ctx)?;
        // TODO: check if other collections or documents still have a object refercnce to this project
        // if not we can safely remove the project
        // if there are still refercnces return an error
        let _ = database(ctx)?.delete_work_report(id).await?;

        Ok(true)
    }
}
