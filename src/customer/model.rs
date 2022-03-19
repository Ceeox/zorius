use async_graphql::{ComplexObject, Context, InputObject, Object, SimpleObject};

use entity::{customer::Model, project};
use sea_orm::{prelude::DateTimeUtc, ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    api::{database, MutationType},
    errors::Result,
    project::model::Project,
};

#[derive(Serialize, Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Customer {
    pub id: Uuid,
    pub name: String,
    pub identifier: String,
    pub note: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,
}

#[ComplexObject]
impl Customer {
    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>> {
        let db = database(ctx)?;
        let models = project::Entity::find()
            .filter(project::Column::CustomerId.eq(self.id))
            .all(db)
            .await?;

        Ok(models.into_iter().map(Project::from).collect())
    }
}

impl From<Model> for Customer {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            identifier: model.identifier,
            note: model.note,
            created_at: model.created_at,
            updated_at: model.updated_at,
            deleted_at: model.deleted_at,
        }
    }
}

#[derive(Serialize, Debug, InputObject, Default)]
pub struct ListCustomerOptions {
    pub ids: Option<Vec<Uuid>>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub first: Option<i32>,
    pub last: Option<i32>,
}

#[derive(Debug, Default)]
pub struct DbListOptions {
    pub ids: Option<Vec<Uuid>>,
    pub start: u64,
    pub limit: u64,
}

#[derive(Serialize, Debug, Clone, InputObject)]
pub struct NewCustomer {
    pub name: String,
    pub identifier: String,
    pub note: Option<String>,
    pub project_ids: Option<Vec<Uuid>>,
}

#[derive(Serialize, InputObject)]
pub struct UpdateCustomer {
    pub name: Option<String>,
    pub identifier: Option<String>,
    pub note: Option<Option<String>>,
}

#[derive(Clone)]
pub struct CustomerChanged {
    pub mutation_type: MutationType,
    pub id: Uuid,
}

#[Object]
impl CustomerChanged {
    async fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }

    async fn id(&self) -> &Uuid {
        &self.id
    }
}
