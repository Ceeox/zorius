use std::{collections::HashMap, sync::Arc};

use crate::api::{database, is_autherized};
use async_graphql::{guard::Guard, Context, Result};
use bson::{doc, from_document, oid::ObjectId};
use futures::lock::Mutex;
use serde::{Deserialize, Serialize};

use super::user::UserId;

static MDB_COLL_ROLES: &str = "roles";

#[derive(Debug, Serialize, Deserialize)]
pub struct Roles {
    pub user_id: UserId,
    pub roles: Vec<Role>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
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
    user_roles: Arc<Mutex<HashMap<ObjectId, Vec<Role>>>>,
}

impl RoleCache {
    pub fn new() -> Self {
        Self {
            user_roles: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn has_role(&self, ctx: &Context<'_>, user_id: &UserId, role: &Role) -> Result<bool> {
        let mut lock = self.user_roles.lock().await;
        println!("{:#?}", lock);
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
        let db = database(ctx)?;
        let filter = doc! { "user_id" : user_id.clone() };
        match db.collection(MDB_COLL_ROLES).find_one(filter, None).await? {
            Some(r) => {
                let roles = from_document::<Roles>(r)?;
                Ok(Some(roles.roles))
            }
            _ => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let user_id = is_autherized(ctx)?;
        match ctx.data_opt::<RoleCache>() {
            Some(role_cache) if role_cache.has_role(ctx, &user_id, &self.role).await? => Ok(()),
            _ => Err("Forbidden".into()),
        }
    }
}
