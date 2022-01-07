use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Result,
};
use futures::{stream, StreamExt};
use uuid::Uuid;

use crate::{
    api::{calc_list_params, claim::Claim, database},
    models::work_report::{
        count_work_reports, delete_work_report, list_work_reports, new_work_report,
        work_report_by_id,
    },
    view::work_report::{NewWorkReport, WorkReport, WorkReportUpdate},
};

#[derive(Default)]
pub struct WorkReportQuery;

#[Object]
impl WorkReportQuery {
    async fn get_workreport_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        mut user_id: Option<Uuid>,
    ) -> Result<Option<WorkReport>> {
        let claim = Claim::from_ctx(ctx)?;
        let db = database(ctx)?.db();
        if user_id.is_none() {
            user_id = Some(claim.user_id());
        }

        work_report_by_id(db, id, user_id).await?;

        Ok(None)
    }

    async fn list_work_reports(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, Option<WorkReport>, EmptyFields, EmptyFields>> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();
        let db = database(ctx)?.db();
        let count = count_work_reports(db, user_id).await?;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let (start, end, limit) = calc_list_params(count, after, before, first, last);

                let work_reports = list_work_reports(db, user_id).await?;

                let mut connection = Connection::new(start > 0, end < count);
                connection
                    .append_stream(
                        stream::iter(work_reports)
                            .enumerate()
                            .map(|(n, wr)| Edge::new(n + start, None)),
                    )
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
    async fn new_work_report(
        &self,
        ctx: &Context<'_>,
        data: NewWorkReport,
    ) -> Result<Option<WorkReport>> {
        let claim = Claim::from_ctx(ctx)?;
        let db = database(ctx)?.db();
        let user_id = claim.user_id();
        let new_work_report = new_work_report(db, user_id, data).await?;
        println!("{:?}", new_work_report);

        Ok(None)
    }

    async fn update_work_report(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        data: WorkReportUpdate,
    ) -> Result<Option<WorkReport>> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();
        Ok(None)
    }

    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::MerchandiseModerator")
    // )))]
    async fn delete_work_report(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();
        let db = database(ctx)?.db();

        Ok(delete_work_report(db, id, user_id).await? >= 1)
    }
}
