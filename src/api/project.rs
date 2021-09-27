use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Result,
};
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::{
    api::{calc_list_params, claim::Claim, database},
    models::project::{count_projects, delete_project, list_projects, new_project, project_by_id},
    view::project::{NewProject, Project},
};

#[derive(Default)]
pub struct ProjectQuery;

#[Object]
impl ProjectQuery {
    async fn get_project_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Project>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = &database(ctx)?.db();
        if let Some(project) = project_by_id(db, id).await? {
            return Ok(Some(project.into()));
        }
        Ok(None)
    }

    async fn list_projects(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, Project, EmptyFields, EmptyFields>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = &database(ctx)?.db();
        let count = count_projects(db).await? as usize;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let (start, end, limit) = calc_list_params(count, after, before, first, last);

                let projects = list_projects(db).await?;

                let mut connection = Connection::new(start > 0, end < count);
                connection
                    .append_stream(
                        stream::iter(projects)
                            .enumerate()
                            .map(|(n, project)| Edge::new(n + start, project.into())),
                    )
                    .await;
                Ok(connection)
            },
        )
        .await
    }
}

#[derive(Default)]
pub struct ProjectMutation;

#[Object]
impl ProjectMutation {
    async fn new_project(&self, ctx: &Context<'_>, new: NewProject) -> Result<Option<Project>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = &database(ctx)?.db();
        if let Some(project) = new_project(db, new).await? {
            return Ok(Some(project.into()));
        }
        Ok(None)
    }

    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::MerchandiseModerator")
    // )))]
    async fn delete_project(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let _ = Claim::from_ctx(ctx)?;
        let db = &database(ctx)?.db();
        Ok(delete_project(db, id).await? >= 1)
    }
}
