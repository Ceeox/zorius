use async_graphql::{guard::Guard, Context, Error, Object, Result};
use bson::{doc, from_document};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};

use crate::{
    api::{claim::Claim, database, MDB_COLL_ROLES},
    models::{
        roles::{Role, RoleCache, RoleGuard, RoleUpdateMode, Roles},
        user::UserId,
    },
};

#[derive(Default)]
pub struct RoleQuery;

#[Object]
impl RoleQuery {
    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::RoleModerator")
    )))]
    async fn list_roles(&self, ctx: &Context<'_>, user_id: UserId) -> Result<Option<Roles>> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_ROLES);
        let filter = doc! {
            "user_id": user_id
        };
        match collection.find_one(filter, None).await? {
            Some(r) => Ok(Some(from_document(r)?)),
            None => Err(Error::new("user in roles not found")),
        }
    }
}

#[derive(Default)]
pub struct RoleMutation;

#[Object]
impl RoleMutation {
    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::RoleModerator")
    )))]
    async fn update_role(
        &self,
        ctx: &Context<'_>,
        user_id: UserId,
        mode: RoleUpdateMode,
        role: Role,
    ) -> Result<Roles> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_ROLES);
        let filter = doc! { "user_id": user_id.clone() };

        let mut update = match mode {
            RoleUpdateMode::Add => doc! {"$push": {"roles": role.to_string()}},
            RoleUpdateMode::Remove => doc! {"$pull": {"roles": role.to_string()}},
        };
        update.insert("$setOnInsert", doc! { "user_id": user_id.clone() });
        println!("{:#?}", update);

        let options = FindOneAndUpdateOptions::builder()
            .return_document(Some(ReturnDocument::After))
            .upsert(Some(true))
            .build();

        let user = match collection
            .find_one_and_update(filter, update, Some(options))
            .await?
        {
            None => return Err(Error::new("user in roles not found")),
            Some(r) => r,
        };

        match ctx.data_opt::<RoleCache>() {
            Some(role_cache) => role_cache.update_rolecache(&user_id, &mode, &role).await,
            _ => {}
        }

        Ok(from_document(user)?)
    }
}
