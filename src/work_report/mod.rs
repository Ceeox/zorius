use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Subscription,
};
use futures::{stream, StreamExt};
use futures_util::Stream;
use uuid::Uuid;

use crate::{
    api::{database, MutationType},
    claim::Claim,
    errors::{Error, Result},
    guards::TokenGuard,
    simple_broker::SimpleBroker,
};

use self::{
    db::{
        count_work_reports, delete_work_report, list_work_reports, new_work_report,
        update_work_report,
    },
    model::{
        DbListOptions, ListWorkReportOptions, NewWorkReport, WorkReport, WorkReportChanged,
        WorkReportUpdate,
    },
};

mod db;
pub mod model;

#[derive(Default)]
pub struct WorkReportQuery;

#[Object]
impl WorkReportQuery {
    #[graphql(guard = "TokenGuard")]
    async fn work_reports(
        &self,
        ctx: &Context<'_>,
        options: Option<ListWorkReportOptions>,
    ) -> Result<Connection<usize, WorkReport, EmptyFields, EmptyFields>> {
        let claim = Claim::from_ctx(ctx)?;
        let mut options = options.unwrap_or_default();
        let user_id = claim.user_id()?;
        options.for_user_id = Some(options.for_user_id.unwrap_or(user_id));
        let db = database(ctx)?;
        let count = count_work_reports(db, user_id).await?;
        let mut db_options = DbListOptions {
            ids: options.ids,
            for_user_id: user_id,
            for_customer_id: options.for_customer_id,
            start_date: options.start_date,
            end_date: options.end_date,
            ..Default::default()
        };

        Ok(query(
            options.after,
            options.before,
            options.first,
            options.last,
            |after, before, first, last| async move {
                let mut start = after.map(|after| after + 1).unwrap_or(0);
                let mut end = before.unwrap_or(10);
                if let Some(first) = first {
                    end = (start + first).min(end);
                }
                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last };
                }
                db_options.start = start as u64;
                db_options.limit = end as u64;

                let work_reports = list_work_reports(db, db_options).await?;

                let mut connection = Connection::new(start > 0, end < count);
                connection
                    .append_stream(
                        stream::iter(work_reports)
                            .enumerate()
                            .map(|(n, wr)| Edge::new(n + start, WorkReport::from(wr))),
                    )
                    .await;
                Ok::<_, Error>(connection)
            },
        )
        .await?)
    }
}

#[derive(Default)]
pub struct WorkReportMutation;

#[Object]
impl WorkReportMutation {
    #[graphql(guard = "TokenGuard")]
    async fn new_work_report(
        &self,
        ctx: &Context<'_>,
        data: NewWorkReport,
    ) -> Result<Option<WorkReport>> {
        let claim = Claim::from_ctx(ctx)?;
        let db = database(ctx)?;
        let user_id = claim.user_id()?;
        let wr = new_work_report(db, user_id, data).await?;
        if let Some(wr) = wr {
            SimpleBroker::publish(WorkReportChanged {
                mutation_type: MutationType::Created,
                id: wr.id,
            });
            Ok(Some(WorkReport::from(wr)))
        } else {
            Ok(None)
        }
    }

    #[graphql(guard = "TokenGuard")]
    async fn update_work_report(
        &self,
        ctx: &Context<'_>,
        mut update: WorkReportUpdate,
    ) -> Result<Option<WorkReport>> {
        let claim = Claim::from_ctx(ctx)?;
        let db = database(ctx)?;
        update.for_user_id = Some(update.for_user_id.unwrap_or(claim.user_id()?));
        let wr = update_work_report(db, update).await?;
        if let Some(wr) = wr {
            SimpleBroker::publish(WorkReportChanged {
                mutation_type: MutationType::Updated,
                id: wr.id,
            });
            Ok(Some(WorkReport::from(wr)))
        } else {
            Ok(None)
        }
    }

    #[graphql(guard = "TokenGuard")]
    async fn delete_work_report(&self, ctx: &Context<'_>, id: Uuid) -> Result<u64> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id()?;
        let db = database(ctx)?;

        Ok(delete_work_report(db, id, user_id).await?)
    }
}

#[derive(Debug, Default, Clone)]
pub struct WorkReportSubscription;

#[Subscription]
impl WorkReportSubscription {
    #[graphql(guard = "TokenGuard")]
    async fn users(
        &self,
        mutation_type: Option<MutationType>,
    ) -> impl Stream<Item = WorkReportChanged> {
        SimpleBroker::<WorkReportChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}
