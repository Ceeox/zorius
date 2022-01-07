use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    sync::Arc,
};

use crate::api::claim::Claim;
use async_graphql::{guard::Guard, Context, Enum, Result, SimpleObject};
use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Roles {
    pub user_id: UserId,
    pub roles: Vec<Role>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, Enum)]
pub enum RoleUpdateMode {
    Add,
    Remove,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize, Enum, Type)]
pub enum Role {
    WorkReportModerator,
    WorkAccountModerator,
    RoleModerator,
    MerchandiseModerator,
    Admin,
    NoRole,
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Role::WorkAccountModerator => write!(f, "WorkAccountModerator"),
            Role::WorkReportModerator => write!(f, "WorkReportModerator"),
            Role::RoleModerator => write!(f, "RoleModerator"),
            Role::Admin => write!(f, "Admin"),
            Role::NoRole => write!(f, "NoRole"),
            Role::MerchandiseModerator => {
                write!(f, "MerchandiseModerator")
            }
        }
    }
}

pub struct RoleGuard {
    pub role: Role,
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();

        match ctx.data_opt::<RoleCache>() {
            Some(role_cache) if role_cache.has_role(ctx, &user_id, &self.role).await? => Ok(()),
            _ => Err("Forbidden".into()),
        }
    }
}

pub struct RoleCache {
    user_roles: Arc<Mutex<HashMap<Uuid, Vec<Role>>>>,
}

impl RoleCache {
    pub fn new() -> Self {
        Self {
            user_roles: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn update_rolecache(&self, user_id: &UserId, mode: &RoleUpdateMode, role: &Role) {
        let mut lock = self.user_roles.lock().await;
        match mode {
            RoleUpdateMode::Add => {
                if let Some(roles) = lock.get_mut(user_id) {
                    roles.push(*role);
                }
            }
            RoleUpdateMode::Remove => {
                if let Some(roles) = lock.get_mut(user_id) {
                    roles.retain(|r| !r.eq(role));
                }
            }
        }
    }

    pub async fn has_role(&self, ctx: &Context<'_>, user_id: &UserId, role: &Role) -> Result<bool> {
        let mut lock = self.user_roles.lock().await;
        match lock.get(user_id) {
            Some(roles) => Ok(roles.contains(&role)),
            None => match self.load_roles(ctx, user_id).await? {
                Some(roles) => {
                    lock.insert(user_id.clone(), roles);
                    let user_roles = lock.get(user_id).unwrap();
                    Ok(user_roles.contains(role))
                }
                None => Ok(false),
            },
        }
    }

    async fn load_roles(&self, ctx: &Context<'_>, user_id: &UserId) -> Result<Option<Vec<Role>>> {
        // TODO: create call in Database class
        Ok(None)
    }
}
