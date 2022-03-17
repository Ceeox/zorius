use sea_orm::prelude::*;

use crate::work_report;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "time_records")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub work_report_id: Uuid,
    pub start: DateTimeUtc,
    pub end: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "work_report::Entity",
        from = "Column::WorkReportId",
        to = "work_report::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    WorkReport,
}

impl Related<work_report::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkReport.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
