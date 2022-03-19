use entity::project::{ActiveModel, Column, Entity, Model};
use migration::sea_query::{Expr, IntoCondition};
use sea_orm::{prelude::*, Condition, DatabaseConnection, Order, QueryOrder, QuerySelect, Set};
use uuid::Uuid;

use super::model::{DbListOptions, NewProject};

pub async fn new_project(
    db: &DatabaseConnection,
    update: NewProject,
) -> Result<Option<Model>, sea_orm::error::DbErr> {
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
) -> Result<Option<Model>, sea_orm::error::DbErr> {
    Ok(Entity::find_by_id(id).one(db).await?)
}

pub async fn count_projects(db: &DatabaseConnection) -> Result<usize, sea_orm::error::DbErr> {
    Ok(Entity::find().count(db).await?)
}

pub async fn list_projects(
    db: &DatabaseConnection,
    options: DbListOptions,
) -> Result<Vec<Model>, sea_orm::error::DbErr> {
    if let Some(ids) = options.ids {
        let con = ids.into_iter().fold(Condition::all(), |acc, id| {
            acc.add(Expr::col(Column::Id).eq(id)).into_condition()
        });
        return Entity::find()
            .filter(con)
            .order_by(Column::CreatedAt, Order::Asc)
            .all(db)
            .await;
    }

    Ok(Entity::find()
        .offset(options.start)
        .limit(options.limit)
        .order_by(Column::CreatedAt, Order::Asc)
        .all(db)
        .await?)
}

pub async fn delete_project(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<u64, sea_orm::error::DbErr> {
    let project: ActiveModel = match project_by_id(db, id).await? {
        Some(project) => project.into(),
        None => return Ok(0),
    };
    let res = Entity::delete(project).exec(db).await?;
    Ok(res.rows_affected)
}