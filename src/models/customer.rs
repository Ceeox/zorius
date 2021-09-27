use sea_orm::{prelude::*, DatabaseConnection, Set};
use uuid::Uuid;

use crate::{
    models::project,
    view::customer::{NewCustomer, UpdateCustomer},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "customers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub identifier: String,
    pub note: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "project::Entity")]
    Projects,
}

impl Related<project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn new_customer(
    db: &DatabaseConnection,
    update: NewCustomer,
) -> Result<Option<(Model, Vec<project::Model>)>, sea_orm::error::DbErr> {
    let customer_id = ActiveModel {
        identifier: Set(update.identifier),
        name: Set(update.name),
        note: Set(update.note),
        ..Default::default()
    };
    let customer_id = Entity::insert(customer_id).exec(db).await?.last_insert_id;
    Ok(customer_by_id(db, customer_id).await?)
}

pub async fn customer_by_id(
    db: &DatabaseConnection,
    id: uuid::Uuid,
) -> Result<Option<(Model, Vec<project::Model>)>, sea_orm::error::DbErr> {
    let customer = Entity::find_by_id(id).one(db).await?;

    if let Some(customer) = customer {
        let projects = customer.find_related(project::Entity).all(db).await?;
        return Ok(Some((customer, projects)));
    }
    Ok(None)
}

pub async fn count_customers(db: &DatabaseConnection) -> Result<usize, sea_orm::error::DbErr> {
    Ok(Entity::find().count(db).await?)
}

pub async fn list_customers(
    db: &DatabaseConnection,
) -> Result<Vec<(Model, Vec<project::Model>)>, sea_orm::error::DbErr> {
    Ok(Entity::find()
        .find_with_related(project::Entity)
        .all(db)
        .await?)
}

pub async fn delete_customer(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<u64, sea_orm::error::DbErr> {
    let customer: ActiveModel = match Entity::find_by_id(id).one(db).await? {
        Some(customer) => customer.into(),
        None => return Ok(0),
    };
    Ok(Entity::delete(customer).exec(db).await?.rows_affected)
}

pub async fn update_customer(
    db: &DatabaseConnection,
    id: Uuid,
    update: UpdateCustomer,
) -> Result<Option<(Model, Vec<project::Model>)>, sea_orm::error::DbErr> {
    let customer = Entity::find_by_id(id).one(db).await?;
    if let Some(customer) = customer {
        let mut customer: ActiveModel = customer.into();
        if let Some(name) = update.name {
            customer.name = Set(name)
        }
        if let Some(identifier) = update.identifier {
            customer.identifier = Set(identifier)
        }
        if let Some(note) = update.note {
            customer.note = Set(note)
        }
        customer.update(db).await?;
        return Ok(customer_by_id(db, id).await?);
    }
    Ok(None)
}
