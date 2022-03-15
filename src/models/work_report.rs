use chrono::Utc;
use entity::{
    customer, project, user,
    work_report::{ActiveModel, Column, Entity, Model},
};
use sea_orm::{prelude::*, DatabaseConnection, Set};
use uuid::Uuid;

use crate::view::work_report::{NewWorkReport, WorkReportUpdate};

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
        ..Default::default()
    };
    let id = Entity::insert(new_work_report)
        .exec(db)
        .await?
        .last_insert_id;
    Ok(work_report_by_id(db, id, owner_id).await?)
}

pub async fn work_report_by_id(
    db: &DatabaseConnection,
    id: Uuid,
    user_id: Uuid,
) -> Result<Option<Model>, sea_orm::error::DbErr> {
    let wr = Entity::find_by_id(id)
        .filter(Column::OwnerId.eq(user_id))
        .one(db)
        .await?;

    if let Some(wr) = wr {
        let owner = wr.find_related(user::Entity).one(db);
        let customer = wr.find_related(customer::Entity).one(db);
        let owner = wr.find_related(project::Entity).one(db);

        Ok(Some(wr))
    } else {
        Ok(None)
    }
}

pub async fn list_work_reports(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Vec<Model>, sea_orm::error::DbErr> {
    Ok(Entity::find()
        .filter(Column::OwnerId.eq(user_id))
        .all(db)
        .await?)
}

pub async fn count_work_reports(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<usize, sea_orm::error::DbErr> {
    Ok(Entity::find()
        .filter(Column::OwnerId.eq(user_id))
        .count(db)
        .await?)
}

#[allow(dead_code)]
pub async fn update_work_report(
    db: &DatabaseConnection,
    id: Uuid,
    user_id: Uuid,
    update: WorkReportUpdate,
) -> Result<Option<Model>, sea_orm::error::DbErr> {
    let model = Entity::find_by_id(id)
        .filter(Column::OwnerId.eq(user_id))
        .one(db)
        .await?;
    if let Some(wr) = model {
        let mut wr: ActiveModel = wr.into();
        if update.customer.is_some() {
            wr.customer_id = Set(update.customer.unwrap());
        }
        if update.description.is_some() {
            wr.description = Set(update.description.unwrap());
        }
        if update.invoiced.is_some() {
            wr.invoiced = Set(update.invoiced.unwrap());
        }
        let _ = wr.update(db);
        return work_report_by_id(db, id, user_id).await;
    }

    Ok(None)
}

pub async fn delete_work_report(
    db: &DatabaseConnection,
    id: Uuid,
    user_id: Uuid,
) -> Result<u64, sea_orm::error::DbErr> {
    let merch: ActiveModel = match Entity::find_by_id(id)
        .filter(Column::OwnerId.eq(user_id))
        .one(db)
        .await?
    {
        Some(merch) => merch.into(),
        None => return Ok(0),
    };
    Ok(Entity::delete(merch).exec(db).await?.rows_affected)
}
