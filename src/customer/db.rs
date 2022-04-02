use entity::customer::{ActiveModel, Column, Entity, Model};
use migration::sea_query::{Expr, IntoCondition};
use sea_orm::{prelude::*, Condition, DatabaseConnection, Order, QueryOrder, QuerySelect, Set};
use uuid::Uuid;

use super::model::{DbListOptions, NewCustomer, UpdateCustomer};

pub async fn new_customer(
    db: &DatabaseConnection,
    update: NewCustomer,
) -> Result<Option<Model>, sea_orm::error::DbErr> {
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
) -> Result<Option<Model>, sea_orm::error::DbErr> {
    Ok(Entity::find_by_id(id).one(db).await?)
}

pub async fn count_customers(db: &DatabaseConnection) -> Result<usize, sea_orm::error::DbErr> {
    Ok(Entity::find().count(db).await?)
}

pub async fn list_customers(
    db: &DatabaseConnection,
    options: DbListOptions,
) -> Result<Vec<Model>, sea_orm::error::DbErr> {
    let mut entity = Entity::find();

    if let Some(ids) = options.ids {
        let con = ids.into_iter().fold(Condition::all(), |acc, id| {
            acc.add(Expr::col(Column::Id).eq(id)).into_condition()
        });

        entity = entity.filter(con);
    }

    Ok(entity
        .offset(options.start)
        .limit(options.limit)
        .order_by(Column::CreatedAt, Order::Asc)
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
) -> Result<Option<Model>, sea_orm::error::DbErr> {
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
        return customer_by_id(db, id).await;
    }
    Ok(None)
}
