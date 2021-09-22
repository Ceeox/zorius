use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Result,
};
use futures::stream::{self, StreamExt};

use crate::{
    api::{calc_list_params, claim::Claim, database},
    models::project::{ProjectEntity, ProjectId},
    view::project::{NewProject, Project},
};

#[derive(Default)]
pub struct ProjectQuery;

#[Object]
impl ProjectQuery {
    async fn get_project_by_id(&self, ctx: &Context<'_>, id: ProjectId) -> Result<Project> {
        let _ = Claim::from_ctx(ctx)?;
        let pool = &database(ctx)?.get_pool();
        Ok(ProjectEntity::get_project_by_id(pool, id).await?.into())
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
        let pool = &database(ctx)?.get_pool();
        let count = ProjectEntity::count_projects(pool).await? as usize;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let (start, end, limit) = calc_list_params(count, after, before, first, last);

                let projects =
                    ProjectEntity::list_projects(pool, start as i64, limit as i64).await?;

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
    async fn new_project(&self, ctx: &Context<'_>, new_project: NewProject) -> Result<Project> {
        let _ = Claim::from_ctx(ctx)?;
        let pool = &database(ctx)?.get_pool();
        Ok(ProjectEntity::new(pool, new_project).await?.into())
    }

    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::MerchandiseModerator")
    // )))]
    async fn delete_project(&self, ctx: &Context<'_>, id: ProjectId) -> Result<Project> {
        let _ = Claim::from_ctx(ctx)?;
        let pool = &database(ctx)?.get_pool();
        Ok(ProjectEntity::delete_project(pool, id).await?.into())
    }
}
