use std::vec;

use entity::{
    customer, project, time_record, user,
    work_report::{ActiveModel, Column, Entity, Model},
};
use log::debug;
use migration::sea_query::{Expr, IntoCondition};
use sea_orm::{
    prelude::*, Condition, DatabaseConnection, DbErr, Order, QueryOrder, QuerySelect, Set,
};
use uuid::Uuid;

use crate::view::work_report::{NewWorkReport, WorkReportListOptions, WorkReportUpdate};

pub async fn new_work_report(
    db: &DatabaseConnection,
    owner_id: Uuid,
    new: NewWorkReport,
) -> Result<
    Option<(
        Model,
        Option<user::Model>,
        Option<customer::Model>,
        Option<project::Model>,
    )>,
    DbErr,
> {
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
) -> Result<
    Option<(
        Model,
        Option<user::Model>,
        Option<customer::Model>,
        Option<project::Model>,
    )>,
    DbErr,
> {
    let wr = Entity::find_by_id(id)
        .filter(Column::OwnerId.eq(user_id))
        .one(db)
        .await?;

    if let Some(wr) = wr {
        let owner = wr.find_related(user::Entity).one(db).await?;
        let customer = wr.find_related(customer::Entity).one(db).await?;
        let project = wr.find_related(project::Entity).one(db).await?;
        Ok(Some((wr, owner, customer, project)))
    } else {
        Ok(None)
    }
}

pub async fn list_work_reports(
    db: &DatabaseConnection,
    options: WorkReportListOptions,
) -> Result<
    Vec<(
        Model,
        Option<user::Model>,
        Option<customer::Model>,
        Option<project::Model>,
        Option<Vec<time_record::Model>>,
    )>,
    DbErr,
> {
    debug!("WorkReportListOptions: {options:#?}");
    let wrs = Entity::find()
        .filter(Column::OwnerId.eq(options.for_user_id))
        .offset(options.start)
        .limit(options.limit)
        .order_by(Column::CreatedAt, Order::Asc)
        .all(db)
        .await?;

    let owners = if options.inc_owner {
        let owner_con = wrs.iter().fold(Condition::any(), |acc, wr| {
            acc.add(Expr::col(user::Column::Id).eq(wr.owner_id).into_condition())
        });
        Some(user::Entity::find().filter(owner_con).all(db).await?)
    } else {
        None
    };

    let customers = if options.inc_customers {
        let customer_con = wrs.iter().fold(Condition::any(), |acc, wr| {
            acc.add(
                Expr::col(customer::Column::Id)
                    .eq(wr.customer_id)
                    .into_condition(),
            )
        });
        Some(
            customer::Entity::find()
                .filter(customer_con)
                .all(db)
                .await?,
        )
    } else {
        None
    };

    let projects = if options.inc_projects {
        let project_con = wrs.iter().fold(Condition::any(), |acc, wr| {
            acc.add(
                Expr::col(project::Column::Id)
                    .eq(wr.project_id)
                    .into_condition(),
            )
        });
        Some(project::Entity::find().filter(project_con).all(db).await?)
    } else {
        None
    };

    let time_record_con = wrs.iter().fold(Condition::any(), |acc, wr| {
        acc.add(
            Expr::col(time_record::Column::WorkReportId)
                .eq(wr.id)
                .into_condition(),
        )
    });
    let time_records = time_record::Entity::find()
        .filter(time_record_con)
        .all(db)
        .await?;

    let res = wrs.into_iter().fold(Vec::new(), |mut acc, wr| {
        let owner = if let Some(owners) = &owners {
            let pos = owners.iter().position(|o| o.id.eq(&wr.owner_id));
            pos.map(|pos| owners[pos].clone())
        } else {
            None
        };

        let customer = if let Some(customers) = &customers {
            let pos = customers.iter().position(|o| o.id.eq(&wr.customer_id));
            pos.map(|pos| customers[pos].clone())
        } else {
            None
        };
        let project = if let Some(projects) = &projects {
            let project_pos = projects.iter().position(|o| Some(o.id).eq(&wr.project_id));
            project_pos.map(|pos| projects[pos].clone())
        } else {
            None
        };
        let time_records: Vec<time_record::Model> = time_records
            .iter()
            .filter(|tr| tr.work_report_id.eq(&wr.id))
            .map(Clone::clone)
            .collect();
        // is there a way to not use clone()?
        acc.push((wr, owner, customer, project, Some(time_records)));
        acc
    });

    Ok(res)
}

pub async fn count_work_reports(db: &DatabaseConnection, user_id: Uuid) -> Result<usize, DbErr> {
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
) -> Result<
    Option<(
        Model,
        Option<user::Model>,
        Option<customer::Model>,
        Option<project::Model>,
    )>,
    DbErr,
> {
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
) -> Result<u64, DbErr> {
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
