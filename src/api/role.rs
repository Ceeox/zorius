use async_graphql::{guard::Guard, Context, Object, Result};

use crate::models::{
    roles::{Role, RoleGuard, RoleUpdateMode, Roles},
    user::UserId,
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
        // TODO: create database function
        unimplemented!()
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
        // TODO: create database function
        unimplemented!()
    }
}
