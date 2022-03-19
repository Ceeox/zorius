use chrono::Utc;
use futures::Future;
use sea_orm::{prelude::*, DatabaseConnection, Order, QueryOrder, QuerySelect, Set};
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
        from = "Column::ControllerId",
        to = "users::Column::Id"
    )]
    Controller,
    #[sea_orm(
        belongs_to = "users::Entity",
        from = "Column::ProjectLeaderId",
        to = "users::Column::Id"
    )]
    ProjectLeader,
}

impl ActiveModelBehavior for ActiveModel {}
