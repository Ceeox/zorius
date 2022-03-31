use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Subscription,
};
use futures::stream::{self, StreamExt};
use futures_util::Stream;
use uuid::Uuid;

mod db;
pub mod model;

use crate::{
    api::{database, MutationType},
    claim::Claim,
    errors::{Error, Result},
    guards::TokenGuard,
    project::model::ProjectChanged,
};

use self::{
    db::{count_projects, delete_project, list_projects, new_project},
    model::{DbListOptions, ListProjectOptions, NewProject, Project},
};

use super::simple_broker::SimpleBroker;

#[derive(Default)]
pub struct ProjectQuery;

#[Object]
impl ProjectQuery {
    #[graphql(guard = "TokenGuard")]
    async fn projects(
        &self,
        ctx: &Context<'_>,
        options: Option<ListProjectOptions>,
    ) -> async_graphql::Result<Connection<usize, Project, EmptyFields, EmptyFields>> {
        let db = &database(ctx)?;
        let count = count_projects(db).await? as usize;
        let options = options.unwrap_or_default();
        let mut db_options = DbListOptions {
            ids: options.ids,
            ..Default::default()
        };

        Ok(query(
            options.after,
            options.before,
            options.first,
            options.last,
            |after, before, first, last| async move {
                let mut start = after.map(|after| after + 1).unwrap_or(0);
                let mut end = before.unwrap_or(count);
                if let Some(first) = first {
                    end = (start + first).min(end);
                }
                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last };
                }
                db_options.start = start as u64;
                db_options.limit = end as u64;

                let projects = list_projects(db, db_options).await?;

                let mut connection = Connection::new(start > 0, end < count);
                connection
                    .append_stream(
                        stream::iter(projects)
                            .enumerate()
                            .map(|(n, project)| Edge::new(n + start, project.into())),
                    )
                    .await;
                Ok::<_, Error>(connection)
            },
        )
        .await?)
    }
}

#[derive(Default)]
pub struct ProjectMutation;

#[Object]
impl ProjectMutation {
    #[graphql(guard = "TokenGuard")]
    async fn new_project(&self, ctx: &Context<'_>, new: NewProject) -> Result<Option<Project>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = &database(ctx)?;
        if let Some(project) = new_project(db, new).await? {
            return Ok(Some(project.into()));
        }
        Ok(None)
    }

    #[graphql(guard = "TokenGuard")]
    async fn delete_project(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let _ = Claim::from_ctx(ctx)?;
        let db = &database(ctx)?;
        Ok(delete_project(db, id).await? >= 1)
    }
}

#[derive(Debug, Default, Clone)]
pub struct ProjectSubscription;

#[Subscription]
impl ProjectSubscription {
    #[graphql(guard = "TokenGuard")]
    async fn projects(
        &self,
        mutation_type: Option<MutationType>,
    ) -> impl Stream<Item = ProjectChanged> {
        SimpleBroker::<ProjectChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}
