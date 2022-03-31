use async_graphql::{ComplexObject, Context, InputObject, Object, SimpleObject};
use entity::{customer, project::Model};
use sea_orm::{prelude::DateTimeUtc, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::{database, MutationType},
    customer::model::Customer,
    errors::Result,
};

#[derive(Serialize, Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Project {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,
}

#[ComplexObject]
impl Project {
    async fn customer(&self, ctx: &Context<'_>) -> Result<Option<Customer>> {
        let db = database(ctx)?;
        let model = customer::Entity::find_by_id(self.customer_id)
            .one(db)
            .await?;
        Ok(model.map(Customer::from))
    }
}

#[derive(Serialize, Debug, InputObject, Default)]
pub struct ListProjectOptions {
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

#[derive(Deserialize, Debug, Clone, InputObject)]
pub struct NewProject {
    pub name: String,
    pub customer_id: Uuid,
    pub note: Option<String>,
}

impl From<Model> for Project {
    fn from(project: Model) -> Self {
        Self {
            id: project.id,
            customer_id: project.customer_id,
            name: project.name,
            note: project.note,
            created_at: project.created_at,
            updated_at: project.updated_at,
            deleted_at: project.deleted_at,
        }
    }
}

#[derive(Clone)]
pub struct ProjectChanged {
    pub mutation_type: MutationType,
    pub id: Uuid,
}

#[Object]
impl ProjectChanged {
    async fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }

    async fn id(&self) -> &Uuid {
        &self.id
    }
}
