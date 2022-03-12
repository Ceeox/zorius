use entity::user::{ActiveModel, Column, Entity, Model};
use uuid::Uuid;

use crate::view::users::{NewUser, OrderBy, OrderDirection, User, UserUpdate};

use sea_orm::{prelude::*, DatabaseConnection, QueryOrder, QuerySelect, Set};

pub async fn new_user(
    db: &DatabaseConnection,
    update: NewUser,
) -> Result<Option<Model>, sea_orm::error::DbErr> {
    let new_user = ActiveModel {
        email: Set(update.email),
        password_hash: Set(User::hash_password(&update.password)),
        name: Set(update.name),
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
    start: usize,
    limit: usize,
) -> Result<Vec<Model>, sea_orm::error::DbErr> {
    let order: Column = order_by.into();
    Ok(Entity::find()
        .offset(start as u64)
        .limit(limit as u64)
        .order_by(order, dic.into())
        .all(db)
        .await?)
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
        user.name = Set(update.name);
        user.update(db).await?;
        return user_by_id(db, id).await;
    }
    Ok(None)
}

pub async fn save_user_avatar(
    db: &DatabaseConnection,
    id: Uuid,
    file: String,
) -> Result<bool, sea_orm::error::DbErr> {
    let user = user_by_id(db, id).await?;
    if let Some(user) = user {
        let mut user: ActiveModel = user.into();
        user.avatar_filename = Set(Some(file));
        user.update(db).await?;
        return Ok(true);
    }
    Ok(false)
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
        return user_by_id(db, id).await;
    }
    Ok(None)
}
