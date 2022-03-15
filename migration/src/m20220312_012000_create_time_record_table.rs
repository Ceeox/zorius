use entity::{time_record::*, user, work_report};

use sea_schema::{
    migration::{sea_query::*, *},
    sea_query::extension::postgres::Type,
};

pub enum TimeRecord {
    Type,
    Drive,
    None,
}

impl Iden for TimeRecord {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Type => "time_record",
                Self::Drive => "drive",
                Self::None => "none",
            }
        )
        .unwrap();
    }
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220312_012000_create_time_record_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TimeRecord::Type)
                    .values([TimeRecord::Drive, TimeRecord::None])
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(Column::Id).integer().primary_key())
                    .col(ColumnDef::new(Column::WorkReportId).uuid().not_null())
                    // .col(
                    //     ColumnDef::new(Column::Type)
                    //         .enumeration("time_record_types", TimeRecord::Type),
                    // )
                    .col(
                        ColumnDef::new(Column::Start)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Column::End).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_time_record-work_report")
                            .from_tbl(Entity)
                            .from_col(Column::WorkReportId)
                            .to_tbl(user::Entity)
                            .to_col(user::Column::Id),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_type(Type::drop().to_owned()).await?;
        manager
            .drop_table(Table::drop().table(work_report::Entity).to_owned())
            .await?;
        Ok(())
    }
}
