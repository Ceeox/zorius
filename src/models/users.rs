use pwhash::sha512_crypt;
use uuid::Uuid;

use crate::view::users::{NewUser, OrderBy, OrderDirection, UserUpdate};

use sea_orm::{prelude::*, DatabaseConnection, Order, QueryOrder, Set};

pub type UserId = Uuid;
pub type UserEmail = String;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub invitation_pending: bool,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub deleted: bool,
}

impl Model {
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    fn hash_password(password: &str) -> String {
        sha512_crypt::hash(password.as_bytes())
            .expect("system random number generator cannot be opened!")
    }

    pub fn is_password_correct(&self, password: &str) -> bool {
        sha512_crypt::verify(password.as_bytes(), &self.password_hash)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn new_user(
    db: &DatabaseConnection,
    update: NewUser,
) -> Result<Option<Model>, sea_orm::error::DbErr> {
    let new_user = ActiveModel {
        email: Set(update.email),
        password_hash: Set(Model::hash_password(&update.password)),
        invitation_pending: Set(false),
        firstname: Set(update.firstname),
        lastname: Set(update.lastname),
        deleted: Set(false),
        ..Default::default()
    };
    let user_id = Entity::insert(new_user).exec(db).await?.last_insert_id;
    Ok(user_by_id(db, user_id).await?)
}

pub async fn user_by_id(
    db: &DatabaseConnection,
    id: uuid::Uuid,
) -> Result<Option<Model>, sea_orm::error::DbErr> {
    Ok(Entity::find_by_id(id).one(db).await?)
}

pub async fn user_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<Model>, sea_orm::error::DbErr> {
    Ok(Entity::find()
        .filter(Column::Email.contains(email))
        .one(db)
        .await?)
}

pub async fn list_users(
    db: &DatabaseConnection,
    order_by: OrderBy,
    dic: OrderDirection,
) -> Result<Vec<Model>, sea_orm::error::DbErr> {
    let order: Column = order_by.into();
    Ok(Entity::find().order_by(order, dic.into()).all(db).await?)
}

pub async fn count_users(db: &DatabaseConnection) -> Result<usize, sea_orm::error::DbErr> {
    Ok(Entity::find().count(db).await?)
}

pub async fn update_user(
    db: &DatabaseConnection,
    id: Uuid,
    update: UserUpdate,
) -> Result<Option<Model>, sea_orm::error::DbErr> {
    let user = user_by_id(db, id).await?;
    if let Some(user) = user {
        let mut user: ActiveModel = user.into();
        user.firstname = Set(update.firstname);
        user.lastname = Set(update.lastname);
        user.update(db).await?;
        return Ok(user_by_id(db, id).await?);
    }
    Ok(None)
}

pub async fn reset_password(
    db: &DatabaseConnection,
    id: Uuid,
    password_hash: &str,
) -> Result<Option<Model>, sea_orm::error::DbErr> {
    let user = user_by_id(db, id).await?;
    if let Some(user) = user {
        let mut user: ActiveModel = user.into();
        user.password_hash = Set(password_hash.to_owned());
        user.update(db).await?;
        return Ok(user_by_id(db, id).await?);
    }
    Ok(None)
}
