use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "time_records")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub work_report_id: Uuid,
    pub start: DateTimeUtc,
    pub end: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
