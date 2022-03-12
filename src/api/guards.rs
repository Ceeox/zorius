use crate::{api::claim::Claim, errors::Error};
use async_graphql::{Context, ErrorExtensions, Guard, Result};

// pub struct AdminGuard;

// #[async_trait::async_trait]
// impl Guard for AdminGuard {
//     async fn check(&self, ctx: &Context<'_>) -> Result<()> {
//         let db = database(ctx)?;
//         let mut res = Err(Error::Forbidden.extend());
//         let claim = Claim::from_ctx(ctx).expect("missing Claim");
//         let user_id: UserId = match claim.user_id() {
//             Ok(r) => r,
//             Err(_) => return res,
//         };
//         let user: Option<User> = match User::find_one(db, USER_COLLECTION, user_id, None).await {
//             Ok(r) => r,
//             _ => return res,
//         };
//         if let Some(user) = user {
//             if user.is_admin {
//                 res = Ok(())
//             }
//         }

//         res
//     }
// }

pub struct TokenGuard;

#[async_trait::async_trait]
impl Guard for TokenGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        match Claim::from_ctx(ctx) {
            Ok(_c) => Ok(()),
            Err(_e) => Err(Error::MissingToken.extend()),
        }
    }
}
