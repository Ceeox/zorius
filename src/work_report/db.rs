use chrono::{Duration, Utc};
use entity::{
    time_record,
    work_report::{ActiveModel, Column, Entity, Model},
};
use migration::sea_query::{Expr, IntoCondition};
use sea_orm::{prelude::*, Condition, DatabaseConnection, Order, QueryOrder, QuerySelect, Set};
use uuid::Uuid;

use crate::errors::{Error, Result};

use super::model::{
    DbListOptions, NewWorkReport, TimeRecordCommand, TimeRecordUpdate, WorkReportUpdate,
};

pub async fn new_work_report(
    db: &DatabaseConnection,
    owner_id: Uuid,
    new: NewWorkReport,
) -> Result<Option<Model>> {
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
) -> Result<Option<Model>> {
    Ok(Entity::find_by_id(id)
        .filter(Column::OwnerId.eq(user_id))
        .one(db)
        .await?)
}

pub async fn list_work_reports(
    db: &DatabaseConnection,
    options: DbListOptions,
) -> Result<Vec<Model>> {
    let mut entity = Entity::find();

    if let Some(ids) = options.ids {
        let con = ids.into_iter().fold(Condition::all(), |acc, id| {
            acc.add(Expr::col(Column::Id).eq(id)).into_condition()
        });

        entity = entity.filter(con);
    }

    if let Some(start) = options.start_date {
        entity = entity.filter(Column::CreatedAt.gte(start));
    }

    if let Some(mut end) = options.end_date {
        // workaround since lte() not includes the end Date, why ever
        end += Duration::days(1);
        entity = entity.filter(Column::CreatedAt.lt(end));
    }

    Ok(entity
        .offset(options.start)
        .limit(options.limit)
        .order_by(Column::CreatedAt, Order::Asc)
        .all(db)
        .await?)
}

pub async fn count_work_reports(db: &DatabaseConnection, user_id: Uuid) -> Result<usize> {
    Ok(Entity::find()
        .filter(Column::OwnerId.eq(user_id))
        .count(db)
        .await?)
}

pub async fn update_work_report(
    db: &DatabaseConnection,
    update: WorkReportUpdate,
) -> Result<Option<Model>> {
    let user_id = update.for_user_id.expect("missing user_id");
    let model = Entity::find_by_id(update.id)
        .filter(Column::OwnerId.eq(user_id))
        .one(db)
        .await?;
    if let Some(wr) = model {
        let mut wr: ActiveModel = wr.into();
        if let Some(customer) = update.customer {
            wr.customer_id = Set(customer);
        }
        if let Some(project) = update.project {
            wr.customer_id = Set(project);
        }
        if let Some(description) = update.description {
            wr.description = Set(description);
        }
        if let Some(invoiced) = update.invoiced {
            wr.invoiced = Set(invoiced);
        }
        if let Some(time_record_update) = update.time_record_update {
            let _ = update_time_record(db, update.id, time_record_update).await?;
        }
        return Ok(Some(wr.update(db).await?));
    }

    Ok(None)
}

pub async fn delete_work_report(db: &DatabaseConnection, id: Uuid, user_id: Uuid) -> Result<u64> {
    let wr = Entity::find_by_id(id)
        .filter(Column::OwnerId.eq(user_id))
        .one(db)
        .await?;
    if let Some(wr) = wr {
        return Ok(wr.delete(db).await?.rows_affected);
    }
    Err(Error::NotFound)
}

pub async fn update_time_record(
    db: &DatabaseConnection,
    work_report_id: Uuid,
    update: TimeRecordUpdate,
) -> Result<Option<time_record::Model>> {
    if let Some(id) = update.id {
        let model = time_record::Entity::find_by_id(id)
            .filter(time_record::Column::WorkReportId.eq(work_report_id))
            .one(db)
            .await?;

        if let Some(model) = model {
            let mut active_model: time_record::ActiveModel = model.into();
            if let Some(start) = update.update_start {
                active_model.start = Set(start);
            }
            if let Some(end) = update.update_end {
                active_model.end = Set(Some(end));
            }
            return Ok(Some(active_model.update(db).await?));
        }
    }

    if let Some(command) = update.command {
        let models = time_record::Entity::find()
            .filter(time_record::Column::WorkReportId.eq(work_report_id))
            .order_by(time_record::Column::Start, Order::Asc)
            .one(db)
            .await?;

        match command {
            TimeRecordCommand::Start => {
                let running = models.into_iter().find(|m| m.end.is_none());
                match running {
                    Some(_) => return Err(Error::TimeRecordStillRunning),
                    None => {
                        let active_model = time_record::ActiveModel {
                            work_report_id: Set(work_report_id),
                            ..Default::default()
                        };
                        return Ok(Some(active_model.insert(db).await?));
                    }
                }
            }
            TimeRecordCommand::End => {
                let running = models.into_iter().find(|m| m.end.is_none());
                if let Some(running) = running {
                    let mut active_model: time_record::ActiveModel = running.into();
                    active_model.end = Set(Some(Utc::now()));
                    return Ok(Some(active_model.update(db).await?));
                } else {
                    return Err(Error::NoTimeRecordRunning);
                }
            }
        }
    }

    Ok(None)
}
