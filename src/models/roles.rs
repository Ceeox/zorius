use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    sync::Arc,
};

use crate::api::{claim::Claim, database};
use async_graphql::{guard::Guard, Context, Enum, Result, SimpleObject};
use bson::{doc, from_document, oid::ObjectId};
use futures::lock::Mutex;
use serde::{Deserialize, Serialize};

use super::user::UserId;
use crate::api::MDB_COLL_ROLES;

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

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize, Enum)]
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
    user_roles: Arc<Mutex<HashMap<ObjectId, Vec<Role>>>>,
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
            RoleUpdateMode::Add => match lock.get_mut(user_id) {
                Some(roles) => roles.push(*role),
                None => {}
            },
            RoleUpdateMode::Remove => match lock.get_mut(user_id) {
                Some(roles) => roles.retain(|r| !r.eq(role)),
                None => {}
            },
        }
    }

    pub async fn has_role(&self, ctx: &Context<'_>, user_id: &UserId, role: &Role) -> Result<bool> {
        let mut lock = self.user_roles.lock().await;
        println!("user_id: {},\nroles: {:#?}", user_id, lock.get(user_id));
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
