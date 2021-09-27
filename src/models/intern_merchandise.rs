use chrono::Utc;
use sea_orm::{prelude::*, DatabaseConnection, Order, QueryOrder, Set};
use uuid::Uuid;

use crate::{
    models::users,
    view::intern_merchandise::{
        IncomingInternMerchandise, InternMerchandise, NewInternMerchandise,
    },
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "intern_merchandises")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub merchandise_id: Option<i64>,
    pub orderer_id: Uuid,
    pub controller_id: Option<Uuid>,
    pub project_leader_id: Uuid,
    pub purchased_on: DateTimeWithTimeZone,
    pub count: i64,
    pub cost: Decimal,
    pub merchandise_name: String,
    pub use_case: Option<String>,
    pub location: Option<String>,
    pub article_number: String,
    pub shop: String,
    pub serial_number: Option<String>,
    pub arrived_on: Option<DateTimeWithTimeZone>,
    pub url: Option<String>,
    pub postage: Option<Decimal>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "users::Entity",
        from = "Column::OrdererId",
        to = "users::Column::Id"
    )]
    Orderer,
    #[sea_orm(
        belongs_to = "users::Entity",
        from = "Column::ProjectLeaderId",
        to = "users::Column::Id"
    )]
    ProjectLeader,
}

#[derive(Debug)]
pub struct UserToInternMerch;

impl Linked for UserToInternMerch {
    type FromEntity = Entity;

    type ToEntity = users::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![Relation::Orderer.def()]
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn new_intern_merch(
    db: &DatabaseConnection,
    orderer_id: Uuid,
    update: NewInternMerchandise,
) -> Result<Option<InternMerchandise>, sea_orm::error::DbErr> {
    let new_intern_merch = ActiveModel {
        orderer_id: Set(orderer_id),
        controller_id: Set(None),
        project_leader_id: Set(update.project_leader_id),
        purchased_on: Set(Utc::now().into()),
        count: Set(update.count),
        cost: Set(update.cost),
        merchandise_name: Set(update.merchandise_name),
        use_case: Set(update.use_case),
        location: Set(update.location),
        article_number: Set(update.article_number),
        shop: Set(update.shop),
        url: Set(update.url),
        postage: Set(Some(update.postage)),
        ..Default::default()
    };
    let intern_merch_id = Entity::insert(new_intern_merch)
        .exec(db)
        .await?
        .last_insert_id;
    Ok(intern_merch_by_id(db, intern_merch_id).await?)
}

pub async fn intern_merch_by_id(
    db: &DatabaseConnection,
    id: uuid::Uuid,
) -> Result<Option<InternMerchandise>, sea_orm::error::DbErr> {
    if let Some(merch) = Entity::find_by_id(id).one(db).await? {
        let ocp: (
            Option<users::Model>,
            Option<users::Model>,
            Option<users::Model>,
        ) = futures::try_join!(
            users::Entity::find_by_id(merch.orderer_id).one(&db),
            users::Entity::find_by_id(merch.controller_id.unwrap_or(Uuid::new_v4())).one(&db),
            users::Entity::find_by_id(merch.project_leader_id).one(&db),
        )?;
        let res = (merch, ocp).into();
        return Ok(Some(res));
    }

    Ok(None)
}

pub async fn count_intern_merch(db: &DatabaseConnection) -> Result<usize, sea_orm::error::DbErr> {
    Ok(Entity::find().count(db).await?)
}

pub async fn list_intern_merch(
    db: &DatabaseConnection,
) -> Result<Vec<Model>, sea_orm::error::DbErr> {
    Ok(Entity::find()
        .order_by(Column::CreatedAt, Order::Asc)
        .all(db)
        .await?)
}

pub async fn delete_intern_merch(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<u64, sea_orm::error::DbErr> {
    let merch: ActiveModel = match Entity::find_by_id(id).one(db).await? {
        Some(merch) => merch.into(),
        None => return Ok(0),
    };
    Ok(Entity::delete(merch).exec(db).await?.rows_affected)
}

pub async fn incoming_intern_merch(
    db: &DatabaseConnection,
    id: Uuid,
    update: IncomingInternMerchandise,
) -> Result<Option<InternMerchandise>, sea_orm::error::DbErr> {
    let merch = Entity::find_by_id(id).one(db).await?;
    if let Some(merch) = merch {
        let mut merch: ActiveModel = merch.into();
        merch.merchandise_id = Set(Some(update.merchandise_id));
        merch.serial_number = Set(Some(update.serial_number));
        merch.update(db).await?;
        return Ok(intern_merch_by_id(db, id).await?);
    }
    Ok(None)
}

// #[derive(Template)]
// #[template(path = "intern_merch_used.html")]
// pub struct StatusTemplate {
//     pub(crate) id: InternMerchandiseId,
//     pub(crate) merchandise_id: Option<i32>,
//     pub(crate) orderer_name: String,
//     pub(crate) count: i32,
//     pub(crate) merchandise_name: String,
//     pub(crate) cost: String,
//     pub(crate) status: InternMerchandiseStatus,
// }
