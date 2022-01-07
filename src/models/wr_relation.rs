use chrono::Utc;
use sea_orm::{prelude::*, DatabaseConnection, JoinType, QuerySelect, Set};
use uuid::Uuid;

use crate::{
    models::{customer, project, users, work_report},
    view::work_report::{NewWorkReport, WorkReportUpdate},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wr_relations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub wr_id: Uuid,
    pub owner_id: Uuid,
    pub customer_id: Uuid,
    pub project_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "work_report::Entity",
        from = "Column::WrId",
        to = "work_report::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    WorkReport,
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

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug)]
pub struct WorkReportRelations;

impl Linked for WorkReportRelations {
    type FromEntity = Entity;

    type ToEntity = work_report::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![
            Relation::WorkReport.def(),
            Relation::Owner.def(),
            Relation::Customer.def(),
            Relation::Project.def(),
        ]
    }
}
