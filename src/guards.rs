use crate::{api::database, claim::Claim, errors::Error};
use async_graphql::{Context, ErrorExtensions, Guard, Result};
use entity::user;
use sea_orm::EntityTrait;

pub struct AdminGuard;

#[async_trait::async_trait]
impl Guard for AdminGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id()?;
        let db = database(ctx)?;
        let model = user::Entity::find_by_id(user_id).one(db).await?;

        if let Some(user) = model {
            if user.is_admin {
                return Ok(());
            }
        }
        return Err(Error::Forbidden.extend());
    }
}

pub struct TokenGuard;

#[async_trait::async_trait]
impl Guard for TokenGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id()?;
        let db = database(ctx)?;
        let model = user::Entity::find_by_id(user_id).one(db).await?;

        if model.is_none() {
            Err(Error::ExpiredToken.extend())
        } else {
            Ok(())
        }
    }
}
