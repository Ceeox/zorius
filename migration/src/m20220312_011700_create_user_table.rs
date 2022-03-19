use entity::user::*;

use chrono::Utc;
use sea_schema::migration::{sea_orm::prelude::Uuid, sea_query::*, *};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220312_011700_create_user_table"
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
                            .not_null()
                            .default(Uuid::new_v4())
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Column::Email).text().unique_key().not_null())
                    .col(ColumnDef::new(Column::PasswordHash).text().not_null())
                    .col(ColumnDef::new(Column::AvatarFilename).text())
                    .col(
                        ColumnDef::new(Column::IsAdmin)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Column::Name).text())
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
