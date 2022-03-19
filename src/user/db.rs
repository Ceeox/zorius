use entity::user::{ActiveModel, Column, Entity, Model};
use migration::sea_query::{Expr, IntoCondition};
use sea_orm::{
    prelude::*, Condition, DatabaseConnection, DbErr, Order, QueryOrder, QuerySelect, Set,
};
use uuid::Uuid;

use super::model::{DbListOptions, NewUser, User, UserUpdate};

pub async fn new_user(db: &DatabaseConnection, update: NewUser) -> Result<Option<Model>, DbErr> {
    let new_user = ActiveModel {
        email: Set(update.email),
        password_hash: Set(User::hash_password(&update.password)),
        name: Set(update.name),
        is_admin: Set(update.is_admin.unwrap_or(false)),
        ..Default::default()
    };
    let user_id = Entity::insert(new_user).exec(db).await?.last_insert_id;
    Ok(user_by_id(db, user_id).await?)
}

pub async fn user_by_id(db: &DatabaseConnection, id: uuid::Uuid) -> Result<Option<Model>, DbErr> {
    Ok(Entity::find_by_id(id).one(db).await?)
}

pub async fn user_by_email(db: &DatabaseConnection, email: &str) -> Result<Option<Model>, DbErr> {
    Ok(Entity::find()
        .filter(Column::Email.contains(email))
        .one(db)
        .await?)
}

pub async fn list_users(
    db: &DatabaseConnection,
    options: DbListOptions,
) -> Result<Vec<Model>, DbErr> {
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
        .offset(options.start as u64)
        .limit(options.limit as u64)
        .order_by(Column::CreatedAt, Order::Asc)
        .all(db)
        .await?)
}

pub async fn count_users(db: &DatabaseConnection) -> Result<usize, DbErr> {
    Ok(Entity::find().count(db).await?)
}

pub async fn update_user(
    db: &DatabaseConnection,
    id: Uuid,
    update: UserUpdate,
) -> Result<Option<Model>, DbErr> {
    let user = user_by_id(db, id).await?;
    if let Some(user) = user {
        let mut user: ActiveModel = user.into();
        user.name = Set(update.name);
        if let Some(admin) = update.is_admin {
            user.is_admin = Set(admin);
        }
        user.update(db).await?;
        return user_by_id(db, id).await;
    }
    Ok(None)
}

pub async fn save_user_avatar(
    db: &DatabaseConnection,
    id: Uuid,
    file: String,
) -> Result<bool, DbErr> {
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
) -> Result<Option<Model>, DbErr> {
    let user = user_by_id(db, id).await?;
    if let Some(user) = user {
        let mut user: ActiveModel = user.into();
        user.password_hash = Set(password_hash.to_owned());
        user.update(db).await?;
        return user_by_id(db, id).await;
    }
    Ok(None)
}
