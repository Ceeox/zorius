use chrono::Utc;
use sea_orm::{
    prelude::*, DatabaseBackend, DatabaseConnection, JoinType, QuerySelect, QueryTrait, Set,
};
use uuid::Uuid;

use crate::{
    models::{customer, project, users, wr_relation},
    view::work_report::{NewWorkReport, WorkReportUpdate},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "work_reports")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub owner_id: Uuid,
    pub customer_id: Uuid,
    pub project_id: Option<Uuid>,
    pub description: String,
    pub invoiced: bool,
    pub report_started: DateTimeWithTimeZone,
    pub report_ended: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "users::Entity",
        from = "Column::OwnerId",
        to = "users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Owner,
    #[sea_orm(
        belongs_to = "customer::Entity",
        from = "Column::CustomerId",
        to = "customer::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Customer,
    #[sea_orm(
        belongs_to = "project::Entity",
        from = "Column::ProjectId",
        to = "project::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Project,
}

impl Related<users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl Related<customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug)]
pub struct WorkReportRelations;

impl Linked for WorkReportRelations {
    type FromEntity = Entity;

    type ToEntity = Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![
            Relation::Owner.def(),
            Relation::Customer.def(),
            Relation::Project.def(),
        ]
    }
}

pub async fn new_work_report(
    db: &DatabaseConnection,
    owner_id: Uuid,
    new: NewWorkReport,
) -> Result<Option<(Model, Option<Model>)>, sea_orm::error::DbErr> {
    let new_work_report = ActiveModel {
        owner_id: Set(owner_id),
        customer_id: Set(new.customer_id),
        project_id: Set(new.project_id),
        description: Set(new.description),
        invoiced: Set(new.invoiced),
        report_started: Set(Utc::now().into()),
        report_ended: Set(None),
        ..Default::default()
    };
    let id = Entity::insert(new_work_report)
        .exec(db)
        .await?
        .last_insert_id;
    let wr_relation = wr_relation::ActiveModel {
        wr_id: Set(id),
        owner_id: Set(owner_id),
        customer_id: Set(new.customer_id),
        project_id: Set(new.project_id),
    };

    let _ = wr_relation::Entity::insert(wr_relation).exec(db).await?;
    Ok(work_report_by_id(db, id, Some(owner_id)).await?)
}

pub async fn work_report_by_id(
    db: &DatabaseConnection,
    id: Uuid,
    user_id: Option<Uuid>,
) -> Result<Option<(Model, Option<Model>)>, sea_orm::error::DbErr> {
    let wr = Entity::find_by_id(id).one(db).await?;

    println!("Workreport: {:#?}", &wr);
    Ok(None)
}

pub async fn list_work_reports(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Vec<Model>, sea_orm::error::DbErr> {
    Ok(Entity::find()
        .filter(Column::OwnerId.eq(user_id))
        .all(db)
        .await?)
}

pub async fn count_work_reports(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<usize, sea_orm::error::DbErr> {
    Ok(Entity::find()
        .filter(Column::OwnerId.eq(user_id))
        .count(db)
        .await?)
}

pub async fn update_work_report(
    db: &DatabaseConnection,
    id: Uuid,
    user_id: Uuid,
    update: WorkReportUpdate,
) -> Result<Option<(Model, Option<Model>)>, sea_orm::error::DbErr> {
    let model = Entity::find_by_id(id)
        .filter(Column::OwnerId.eq(user_id))
        .one(db)
        .await?;
    if let Some(wr) = model {
        let mut wr: ActiveModel = wr.into();
        if update.customer.is_some() {
            wr.customer_id = Set(update.customer.unwrap());
        }
        if update.description.is_some() {
            wr.description = Set(update.description.unwrap());
        }
        if update.invoiced.is_some() {
            wr.invoiced = Set(update.invoiced.unwrap());
        }
        if update.report_started.is_some() {
            wr.report_started = Set(update.report_started.unwrap());
        }
        if update.report_ended.is_some() {
            wr.report_ended = Set(update.report_ended);
        }
        wr.update(db);
        return Ok(work_report_by_id(db, id, Some(user_id)).await?);
    }

    Ok(None)
}

pub async fn delete_work_report(
    db: &DatabaseConnection,
    id: Uuid,
    user_id: Uuid,
) -> Result<u64, sea_orm::error::DbErr> {
    let merch: ActiveModel = match Entity::find_by_id(id)
        .filter(Column::OwnerId.eq(user_id))
        .one(db)
        .await?
    {
        Some(merch) => merch.into(),
        None => return Ok(0),
    };
    Ok(Entity::delete(merch).exec(db).await?.rows_affected)
}

mod tests {}
