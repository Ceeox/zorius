use sea_orm::{prelude::*, DatabaseConnection, Order, QueryOrder, Set};
use uuid::Uuid;

use crate::models::customer;
use crate::view::project::NewProject;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub customer_id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "customer::Entity",
        from = "Column::CustomerId",
        to = "customer::Column::Id"
    )]
    Customer,
}

impl Related<customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn new_project(
    db: &DatabaseConnection,
    update: NewProject,
) -> Result<Option<(Model, Option<customer::Model>)>, sea_orm::error::DbErr> {
    let new_project = ActiveModel {
        customer_id: Set(update.customer_id),
        name: Set(update.name),
        note: Set(update.note),
        ..Default::default()
    };
    let project_id = Entity::insert(new_project).exec(db).await?.last_insert_id;
    Ok(project_by_id(db, project_id).await?)
}

pub async fn project_by_id(
    db: &DatabaseConnection,
    id: uuid::Uuid,
) -> Result<Option<(Model, Option<customer::Model>)>, sea_orm::error::DbErr> {
    Ok(Entity::find_by_id(id)
        .find_also_related(customer::Entity)
        .one(db)
        .await?)
}

pub async fn count_projects(db: &DatabaseConnection) -> Result<usize, sea_orm::error::DbErr> {
    Ok(Entity::find().count(db).await?)
}

pub async fn list_projects(
    db: &DatabaseConnection,
) -> Result<Vec<(Model, Option<customer::Model>)>, sea_orm::error::DbErr> {
    Ok(Entity::find()
        .find_also_related(customer::Entity)
        .order_by(Column::CreatedAt, Order::Asc)
        .all(db)
        .await?)
}

pub async fn delete_project(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<u64, sea_orm::error::DbErr> {
    let project: ActiveModel = match project_by_id(db, id).await? {
        Some(project) => project.0.into(),
        None => return Ok(0),
    };
    let res = Entity::delete(project).exec(db).await?;
    Ok(res.rows_affected)
}
