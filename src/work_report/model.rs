use async_graphql::{ComplexObject, Context, Enum, InputObject, Object, SimpleObject};
use entity::{customer, project, time_record, user, work_report::*};
use sea_orm::{
    prelude::{Date, DateTimeUtc},
    ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    api::{database, MutationType},
    customer::model::Customer,
    errors::Result,
    project::model::Project,
    user::model::User,
};

#[derive(SimpleObject, Debug, Serialize, Clone)]
#[graphql(complex)]
pub struct WorkReport {
    pub id: Uuid,
    #[graphql(visible = false)]
    pub owner_id: Uuid,
    #[graphql(visible = false)]
    pub customer_id: Uuid,
    #[graphql(visible = false)]
    pub project_id: Option<Uuid>,
    pub description: String,
    pub invoiced: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[ComplexObject]
impl WorkReport {
    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let db = database(ctx)?;
        let model = user::Entity::find_by_id(self.owner_id).one(db).await?;
        Ok(model.map(User::from))
    }

    async fn customer(&self, ctx: &Context<'_>) -> Result<Option<Customer>> {
        let db = database(ctx)?;
        let model = customer::Entity::find_by_id(self.customer_id)
            .one(db)
            .await?;
        Ok(model.map(Customer::from))
    }

    async fn project(&self, ctx: &Context<'_>) -> Result<Option<Project>> {
        let db = database(ctx)?;
        if let Some(id) = self.project_id {
            let model = project::Entity::find_by_id(id).one(db).await?;
            return Ok(model.map(Project::from));
        }
        Ok(None)
    }

    async fn time_records(&self, ctx: &Context<'_>) -> Result<Vec<TimeRecord>> {
        let db = database(ctx)?;
        let model = time_record::Entity::find()
            .filter(time_record::Column::WorkReportId.eq(self.id))
            .order_by(time_record::Column::Start, Order::Asc)
            .all(db)
            .await?;
        Ok(model.into_iter().map(TimeRecord::from).collect())
    }
}

impl From<Model> for WorkReport {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            owner_id: model.owner_id,
            customer_id: model.customer_id,
            project_id: model.project_id,
            description: model.description,
            invoiced: model.invoiced,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Serialize, Debug, InputObject, Default)]
pub struct ListWorkReportOptions {
    pub ids: Option<Vec<Uuid>>,
    pub for_user_id: Option<Uuid>,
    pub for_customer_id: Option<Uuid>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub after: Option<String>,
    pub before: Option<String>,
    #[graphql(default = 10)]
    pub first: Option<i32>,
    pub last: Option<i32>,
}

#[derive(Serialize, Debug, InputObject)]
pub struct NewWorkReport {
    pub customer_id: Uuid,
    pub project_id: Option<Uuid>,
    pub description: String,
    pub invoiced: bool,
}

#[derive(Debug, Default)]
pub struct DbListOptions {
    pub ids: Option<Vec<Uuid>>,
    pub for_user_id: Uuid,
    pub for_customer_id: Option<Uuid>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub start: u64,
    pub limit: u64,
}

#[derive(Serialize, Debug, InputObject)]
pub struct WorkReportUpdate {
    pub id: Uuid,
    pub for_user_id: Option<Uuid>,
    pub customer: Option<Uuid>,
    pub project: Option<Uuid>,
    pub description: Option<String>,
    pub invoiced: Option<bool>,
    pub start_time_record: Option<bool>,
    pub end_time_record: Option<bool>,
    pub time_record_update: Option<TimeRecordUpdate>,
}
#[derive(Clone)]
pub struct WorkReportChanged {
    pub mutation_type: MutationType,
    pub id: Uuid,
}

#[Object]
impl WorkReportChanged {
    async fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }

    async fn id(&self) -> &Uuid {
        &self.id
    }
}

#[derive(SimpleObject, Debug, Serialize, Clone)]
pub struct TimeRecord {
    pub id: Uuid,
    pub start: DateTimeUtc,
    pub end: Option<DateTimeUtc>,
}

impl From<time_record::Model> for TimeRecord {
    fn from(model: time_record::Model) -> Self {
        Self {
            id: model.id,
            start: model.start,
            end: model.end,
        }
    }
}

#[derive(Serialize, Debug, InputObject)]
pub struct TimeRecordUpdate {
    pub command: Option<TimeRecordCommand>,
    pub id: Option<Uuid>,
    pub update_start: Option<DateTimeUtc>,
    pub update_end: Option<DateTimeUtc>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Copy, Enum)]
pub enum TimeRecordCommand {
    Start,
    End,
}
