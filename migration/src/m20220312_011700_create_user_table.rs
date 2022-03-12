use entity::user;

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
                    .table(user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user::Column::Id)
                            .uuid()
                            .not_null()
                            .default(Uuid::new_v4())
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(user::Column::Email)
                            .text()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(user::Column::PasswordHash).text().not_null())
                    .col(ColumnDef::new(user::Column::AvatarFilename).text())
                    .col(ColumnDef::new(user::Column::Name).text())
                    .col(
                        ColumnDef::new(user::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Utc::now())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user::Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Utc::now())
                            .not_null(),
                    )
                    .col(ColumnDef::new(user::Column::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await?;
        Ok(())
    }
}
