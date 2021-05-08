use async_graphql::{guard::Guard, Context, Error, Object, Result};
use bson::{doc, from_document, to_document};

use crate::{
    api::{claim::Claim, database},
    database::MDB_COLL_WORK_REPORTS,
    models::{
        roles::{Role, RoleGuard},
        work_report::project::{NewProject, Project, ProjectId},
    },
};

#[derive(Default)]
pub struct ProjectQuery;

#[Object]
impl ProjectQuery {
    async fn get_project(
        &self,
        ctx: &Context<'_>,
        project_id: ProjectId,
    ) -> Result<Option<Project>> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_WORK_REPORTS);
        let filter = doc! {
            "_id": project_id
        };
        match collection.find_one(filter, None).await? {
            Some(r) => Ok(Some(from_document(r)?)),
            None => Err(Error::new("customer not found")),
        }
    }
}

#[derive(Default)]
pub struct ProjectMutation;

#[Object]
impl ProjectMutation {
    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::WorkReportModerator")
    )))]
    async fn new_project(&self, ctx: &Context<'_>, new: NewProject) -> Result<Project> {
        let _ = Claim::from_ctx(ctx)?;

        let collection = database(ctx)?.collection(MDB_COLL_WORK_REPORTS);
        let project = Project::new(new);
        let insert = to_document(&project)?;
        let _ = collection.insert_one(insert, None).await?;
        Ok(project)
    }
}
