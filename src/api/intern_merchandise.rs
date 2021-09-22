use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Result,
};
use futures::stream::{self, StreamExt};

use crate::{
    api::{calc_list_params, claim::Claim, database},
    models::intern_merchandise::{InternMerchandiseEntity, InternMerchandiseId},
    view::intern_merchandise::{
        IncomingInternMerchandise, InternMerchandise, NewInternMerchandise,
    },
};

#[derive(Default)]
pub struct InternMerchandiseQuery;

#[Object]
impl InternMerchandiseQuery {
    async fn get_intern_merch_by_id(
        &self,
        ctx: &Context<'_>,
        id: InternMerchandiseId,
    ) -> Result<InternMerchandise> {
        let _ = Claim::from_ctx(ctx)?;
        let pool = &database(ctx)?.get_pool();
        Ok(InternMerchandiseEntity::get_intern_merch_by_id(pool, id)
            .await?
            .into())
    }

    async fn list_intern_merch(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, InternMerchandise, EmptyFields, EmptyFields>> {
        let _ = Claim::from_ctx(ctx)?;
        let pool = &database(ctx)?.get_pool();
        let count = InternMerchandiseEntity::count_intern_merch(pool).await? as usize;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let (start, end, limit) = calc_list_params(count, after, before, first, last);

                let merchs =
                    InternMerchandiseEntity::list_intern_merch(pool, start as i64, limit as i64)
                        .await?;

                let mut connection = Connection::new(start > 0, end < count);
                connection
                    .append_stream(
                        stream::iter(merchs)
                            .enumerate()
                            .map(|(n, merch)| Edge::new(n + start, merch.into())),
                    )
                    .await;
                Ok(connection)
            },
        )
        .await
    }

    pub async fn count_intern_merch(&self, ctx: &Context<'_>) -> Result<usize> {
        Ok(InternMerchandiseEntity::count_intern_merch(&database(ctx)?.get_pool()).await? as usize)
    }
}

#[derive(Default)]
pub struct InternMerchandiseMutation;

#[Object]
impl InternMerchandiseMutation {
    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::MerchandiseModerator")
    // )))]
    async fn new_intern_merch(
        &self,
        ctx: &Context<'_>,
        new: NewInternMerchandise,
    ) -> Result<InternMerchandise> {
        let claim = Claim::from_ctx(ctx)?;
        let orderer_id = claim.user_id();
        let pool = &database(ctx)?.get_pool();
        Ok(InternMerchandiseEntity::new(pool, orderer_id, new)
            .await?
            .into())
    }

    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::MerchandiseModerator")
    // )))]
    async fn incoming_intern_merchandise(
        &self,
        ctx: &Context<'_>,
        update: IncomingInternMerchandise,
    ) -> Result<InternMerchandise> {
        let _ = Claim::from_ctx(ctx)?;
        let pool = &database(ctx)?.get_pool();
        Ok(
            InternMerchandiseEntity::incoming_intern_merchandise(pool, update)
                .await?
                .into(),
        )
    }

    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::MerchandiseModerator")
    // )))]
    async fn delete_intern_merch(
        &self,
        ctx: &Context<'_>,
        id: InternMerchandiseId,
    ) -> Result<bool> {
        let _ = Claim::from_ctx(ctx)?;
        let pool = &database(ctx)?.get_pool();
        let merch = InternMerchandiseEntity::get_intern_merch_by_id(pool, id).await?;
        merch.delete(pool).await?;

        Ok(true)
    }
}
