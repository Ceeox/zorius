use chrono::Utc;
use sea_orm::{prelude::*, DatabaseConnection, Set};
use uuid::Uuid;

use crate::view::work_report::{NewWorkReport, WorkReportUpdate};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "work_reports")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: Uuid,
    owner_id: Uuid,
    customer_id: Uuid,
    project_id: Option<Uuid>,
    description: String,
    invoiced: bool,
    report_started: DateTimeWithTimeZone,
    report_ended: Option<DateTimeWithTimeZone>,
    created_at: DateTimeWithTimeZone,
    updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

// mod users_to_workreport {
//     use crate::models::{customer, project, users, work_report};
//     use sea_orm::prelude::*;

//     #[derive(Clone, Debug, PartialEq, DeriveActiveModel)]
//     pub struct Model {
//         pub owner_id: Uuid,
//         pub customer_id: Uuid,
//         pub project_id: Uuid,
//     }

//     #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
//     pub struct Entity;

//     impl EntityName for Entity {
//         fn schema_name(&self) -> Option<&str> {
//             Some("public")
//         }

//         fn table_name(&self) -> &str {
//             "users_workreport"
//         }
//     }

//     #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
//     pub enum Relation {
//         #[sea_orm(
//             belongs_to = "users::Entity",
//             from = "Column::OwnerId",
//             to = "users::Column::Id"
//         )]
//         Owner,
//         #[sea_orm(
//             belongs_to = "customer::Entity",
//             from = "Column::CustomerId",
//             to = "customer::Column::Id"
//         )]
//         Customer,
//         #[sea_orm(
//             belongs_to = "project::Entity",
//             from = "Column::ProjectId",
//             to = "project::Column::Id"
//         )]
//         Project,
//     }

//     #[derive(Debug)]
//     pub struct UserToInternMerch;

//     impl Linked for UserToInternMerch {
//         type FromEntity = Entity;

//         type ToEntity = work_report::Entity;

//         fn link(&self) -> Vec<sea_orm::LinkDef> {
//             vec![
//                 Relation::Owner.def(),
//                 Relation::Customer.def(),
//                 Relation::Project.def(),
//             ]
//         }
//     }
// }

impl ActiveModelBehavior for ActiveModel {}

pub async fn new_work_report(
    db: &DatabaseConnection,
    owner_id: Uuid,
    new: NewWorkReport,
) -> Result<Option<Model>, sea_orm::error::DbErr> {
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
    let work_report_id = Entity::insert(new_work_report)
        .exec(db)
        .await?
        .last_insert_id;
    Ok(None)
}

pub async fn work_report_by_id(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<Option<Model>, sea_orm::error::DbErr> {
    Ok(Entity::find_by_id(id).one(db).await?)
}

pub async fn list_work_reports(
    db: &DatabaseConnection,
) -> Result<Vec<Model>, sea_orm::error::DbErr> {
    Ok(Entity::find().all(db).await?)
}

pub async fn update_work_report(
    db: &DatabaseConnection,
    id: Uuid,
    update: WorkReportUpdate,
) -> Result<Option<Model>, sqlx::Error> {
    todo!()
}
