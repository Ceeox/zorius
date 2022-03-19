use entity::customer::*;

use chrono::Utc;
use sea_schema::migration::{sea_orm::prelude::Uuid, sea_query::*, *};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220312_011800_create_customer_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .uuid()
                            .default(Uuid::new_v4())
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Column::Name).text().not_null())
                    .col(ColumnDef::new(Column::Identifier).text().not_null())
                    .col(ColumnDef::new(Column::Note).text())
                    .col(
                        ColumnDef::new(Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Utc::now())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Utc::now())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Column::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await?;
        Ok(())
    }
}
