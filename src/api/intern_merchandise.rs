use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Result,
};
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::{
    api::{calc_list_params, claim::Claim, database},
    models::intern_merchandise::{
        count_intern_merch, delete_intern_merch, incoming_intern_merch, intern_merch_by_id,
        list_intern_merch, new_intern_merch,
    },
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
        id: Uuid,
    ) -> Result<Option<InternMerchandise>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = &database(ctx)?.db();
        println!("here");

        if let Some(merch) = intern_merch_by_id(db, id).await? {
            return Ok(Some(merch));
        }
        Ok(None)
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
        let db = &database(ctx)?.db();
        let count = count_intern_merch(db).await?;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let (start, end, limit) = calc_list_params(count, after, before, first, last);

                let merchs = list_intern_merch(db, start, limit).await?;

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
        let _ = Claim::from_ctx(ctx)?;
        let db = database(ctx)?.db();
        Ok(count_intern_merch(db).await?)
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
    ) -> Result<Option<InternMerchandise>> {
        let claim = Claim::from_ctx(ctx)?;
        let orderer_id = claim.user_id();
        let db = &database(ctx)?.db();
        if let Some(merch) = new_intern_merch(db, orderer_id, new).await? {
            return Ok(Some(merch.into()));
        }
        Ok(None)
    }

    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::MerchandiseModerator")
    // )))]
    async fn incoming_intern_merchandise(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        update: IncomingInternMerchandise,
    ) -> Result<Option<InternMerchandise>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = &database(ctx)?.db();
        if let Some(merch) = incoming_intern_merch(db, id, update).await? {
            return Ok(Some(merch.into()));
        }
        Ok(None)
    }

    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::MerchandiseModerator")
    // )))]
    async fn delete_intern_merch(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let _ = Claim::from_ctx(ctx)?;
        let db = &database(ctx)?.db();
        Ok(delete_intern_merch(db, id).await? >= 1)
    }
}
