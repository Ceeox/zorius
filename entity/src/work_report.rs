use chrono::Utc;
use sea_orm::{prelude::*, Set};

use crate::{customer, project, user};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "work_reports")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub owner_id: Uuid,
    pub customer_id: Uuid,
    pub project_id: Option<Uuid>,
    pub description: String,
    pub invoiced: bool,
    pub report_started: DateTimeUtc,
    pub report_ended: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user::Entity",
        from = "Column::OwnerId",
        to = "user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Owner,
    #[sea_orm(
        belongs_to = "customer::Entity",
        from = "Column::CustomerId",
        to = "customer::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Customer,
    #[sea_orm(
        belongs_to = "project::Entity",
        from = "Column::ProjectId",
        to = "project::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Project,
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl Related<customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

pub struct ToWorkReport;

impl Linked for ToWorkReport {
    type FromEntity = Entity;

    type ToEntity = project::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            Relation::Project.def(),
            Relation::Customer.def(),
            Relation::Owner.def(),
        ]
    }
}

impl ActiveModelBehavior for ActiveModel {
    /// Create a new ActiveModel with default values. Also used by `Default::default()`.
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..ActiveModelTrait::default()
        }
    }

    /// Will be triggered before insert / update
    fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
        self.updated_at = Set(Utc::now());
        Ok(self)
    }

    /// Will be triggered after insert / update
    fn after_save(model: Model, _insert: bool) -> Result<Model, DbErr> {
        Ok(model)
    }

    /// Will be triggered before delete
    fn before_delete(self) -> Result<Self, DbErr> {
        Ok(self)
    }

    /// Will be triggered after delete
    fn after_delete(self) -> Result<Self, DbErr> {
        Ok(self)
    }
}
