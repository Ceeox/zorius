use std::collections::HashMap;

use async_graphql::{guard::Guard, Context, Result};
use bson::oid::ObjectId;

use crate::api::is_autherized;

use super::user::UserId;

#[derive(Eq, PartialEq, Clone)]
pub enum Role {
    WorkReportModerator,
    WorkAccountModerator,
    RoleModerator,
    Admin,
    NoRole,
}

pub struct RoleGuard {
    pub role: Role,
}

pub struct RoleCache {
    user_roles: HashMap<ObjectId, Vec<Role>>,
}

impl RoleCache {
    pub fn new() -> Self {
        Self {
            user_roles: HashMap::new(),
        }
    }

    pub fn has_role(&self, ctx: &Context<'_>, user_id: &UserId, role: &Role) -> bool {
        match self.user_roles.get(user_id) {
            Some(roles) => roles.contains(&role),
            None => false,
        }
    }

    fn load_roles(&self, ctx: &Context<'_>, user_id: &UserId) -> Option<Vec<Role>> {
        None
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let user_id = is_autherized(ctx)?;
        match ctx.data_opt::<RoleCache>() {
            Some(role_cache) if role_cache.has_role(ctx, &user_id, &self.role) => Ok(()),
            _ => Err("Forbidden".into()),
        }
    }
}
