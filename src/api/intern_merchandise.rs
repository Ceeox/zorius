use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Result,
};

use crate::{
    api::{claim::Claim, database},
    models::intern_merchandise::{InternMerchandise as DBInternMerchandise, InternMerchandiseId},
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
        let _ = DBInternMerchandise::test(&database(ctx)?.get_pool(), id).await;
        Ok(
            DBInternMerchandise::get_intern_merch_by_id(&database(ctx)?.get_pool(), id)
                .await?
                .into(),
        )
    }

    // async fn get_intern_merch_by_merch_id(
    //     &self,
    //     ctx: &Context<'_>,
    //     merchandise_id: i32,
    // ) -> Result<InternMerchandise> {
    //     let _ = Claim::from_ctx(ctx)?;
    //     Ok(database(ctx)?
    //         .get_intern_merch_by_merch_id(merchandise_id)
    //         .await?)
    // }

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
        let count = DBInternMerchandise::count_intern_merch(pool).await? as usize;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = after
                    .map(|after: usize| after.saturating_add(1))
                    .unwrap_or(0);
                let mut end = before.unwrap_or(count);

                if let Some(first) = first {
                    end = (start.saturating_add(first)).min(end);
                }
                if let Some(last) = last {
                    start = if last > end.saturating_sub(start) {
                        end
                    } else {
                        end.saturating_sub(last)
                    };
                }
                let limit = (end.saturating_sub(start)) as i64;

                let cursor =
                    DBInternMerchandise::list_intern_merch(pool, start as i64, limit).await?;

                let mut connection = Connection::new(start > 0, end < count);
                connection.append(cursor.into_iter().enumerate().map(|(n, merch)| {
                    Edge::with_additional_fields(n + start, merch.into(), EmptyFields)
                }));
                Ok(connection)
            },
        )
        .await
    }

    pub async fn count_intern_merch(&self, ctx: &Context<'_>) -> Result<usize> {
        Ok(DBInternMerchandise::count_intern_merch(&database(ctx)?.get_pool()).await? as usize)
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
        Ok(DBInternMerchandise::new(pool, orderer_id, new)
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
            DBInternMerchandise::incoming_intern_merchandise(pool, update)
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
        let merch = DBInternMerchandise::get_intern_merch_by_id(pool, id).await?;
        merch.delete(pool).await?;

        Ok(true)
    }
}
