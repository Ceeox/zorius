use entity::{customer, project, user, work_report};

use chrono::Utc;
use sea_schema::migration::{sea_orm::prelude::Uuid, sea_query::*, *};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220312_012000_create_work_report_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(work_report::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(work_report::Column::Id)
                            .uuid()
                            .default(Uuid::new_v4())
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(work_report::Column::CustomerId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(work_report::Column::ProjectId).uuid())
                    .col(
                        ColumnDef::new(work_report::Column::OwnerId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(work_report::Column::Description)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(work_report::Column::Invoiced)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(work_report::Column::ReportStarted)
                            .timestamp_with_time_zone()
                            .default(Utc::now())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(work_report::Column::ReportEnded).timestamp_with_time_zone(),
                    )
                    .col(
                        ColumnDef::new(user::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Utc::now())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(work_report::Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Utc::now())
                            .not_null(),
                    )
                    .col(ColumnDef::new(work_report::Column::DeletedAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_work_report-customer")
                            .from_tbl(work_report::Entity)
                            .from_col(work_report::Column::CustomerId)
                            .to_tbl(customer::Entity)
                            .to_col(customer::Column::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_work_report-project")
                            .from_tbl(work_report::Entity)
                            .from_col(work_report::Column::ProjectId)
                            .to_tbl(project::Entity)
                            .to_col(project::Column::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_work_report-owner")
                            .from_tbl(work_report::Entity)
                            .from_col(work_report::Column::OwnerId)
                            .to_tbl(user::Entity)
                            .to_col(user::Column::Id),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(work_report::Entity).to_owned())
            .await?;
        Ok(())
    }
}
