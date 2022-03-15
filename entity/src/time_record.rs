use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "time_records")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub work_report_id: Uuid,
    pub tr_type: TimeRecordType,
    pub start: DateTimeUtc,
    pub end: Option<DateTimeUtc>,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum TimeRecordType {
    #[sea_orm(string_value = "drive")]
    Drive,
    #[sea_orm(string_value = "none")]
    None,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
