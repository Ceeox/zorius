use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    guard::Guard,
    Context, Error, Object, Result,
};
use bson::from_document;
use futures::StreamExt;

use crate::{
    api::{claim::Claim, database2},
    models::{
        project::{NewProject, ProjectId, ProjectResponse, ProjectUpdate},
        roles::{Role, RoleGuard},
    },
};

#[derive(Default)]
pub struct ProjectQuery;

#[Object]
impl ProjectQuery {
    async fn get_project_by_id(&self, ctx: &Context<'_>, id: ProjectId) -> Result<ProjectResponse> {
        let _ = Claim::from_ctx(ctx)?;
        Ok(database2(ctx)?.get_project_by_id(id).await?)
    }

    async fn list_projects(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, ProjectResponse, EmptyFields, EmptyFields>> {
        let _ = Claim::from_ctx(ctx)?;
        let doc_count = database2(ctx)?.count_projects().await?;

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
                let limit = if let Some(x) = end.checked_sub(start) {
                    x as i64
                } else {
                    1i64
                };

                let cursor = database2(ctx)?.list_projects(start as i64, limit).await?;

                let mut connection = Connection::new(start > 0, end < doc_count);
                connection
                    .append_stream(cursor.enumerate().map(|(n, doc)| {
                        let project = from_document::<ProjectResponse>(doc.unwrap()).unwrap();
                        Edge::with_additional_fields(n + start, project, EmptyFields)
                    }))
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
    async fn new_project(&self, ctx: &Context<'_>, new: NewProject) -> Result<ProjectResponse> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();

        Ok(database2(ctx)?.new_project(user_id.clone(), new).await?)
    }

    async fn update_project(
        &self,
        ctx: &Context<'_>,
        id: ProjectId,
        update: ProjectUpdate,
    ) -> Result<ProjectResponse> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();
        let _ = database2(ctx)?
            .update_project(id.clone(), user_id.clone(), update)
            .await?;

        Ok(database2(ctx)?.get_project_by_id(id).await?)
    }

    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::MerchandiseModerator")
    )))]
    async fn delete_project(&self, ctx: &Context<'_>, id: ProjectId) -> Result<bool> {
        let _ = Claim::from_ctx(ctx)?;
        if database2(ctx)?
            .has_ref_to_work_report("project_id", id.clone())
            .await?
        {
            return Err(Error::new(
                "Can not delete Project with still a reference to a WorkReport",
            ));
        }
        let _ = database2(ctx)?.delete_project(id).await?;

        Ok(true)
    }
}
