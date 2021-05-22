use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    guard::Guard,
    Context, Object, Result,
};
use bson::{de::from_document, oid::ObjectId};
use futures::stream::StreamExt;

use crate::{
    api::{claim::Claim, database2},
    models::{
        intern_merchandise::{
            DBInternMerchandise, InternMerchandise, InternMerchandiseId, InternMerchandiseStatus,
            InternMerchandiseUpdate, NewInternMerchandise,
        },
        roles::{Role, RoleGuard},
    },
};

#[derive(Default)]
pub struct InternMerchandiseQuery;

#[Object]
impl InternMerchandiseQuery {
    async fn get_intern_merch_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> Result<InternMerchandise> {
        let _ = Claim::from_ctx(ctx)?;
        Ok(database2(ctx)?.get_intern_merch_by_id(id).await?)
    }

    async fn get_intern_merch_by_merch_id(
        &self,
        ctx: &Context<'_>,
        merchandise_id: i32,
    ) -> Result<InternMerchandise> {
        let _ = Claim::from_ctx(ctx)?;
        Ok(database2(ctx)?
            .get_intern_merch_by_merch_id(merchandise_id)
            .await?)
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
        let doc_count = database2(ctx)?.count_intern_merch().await?;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = after.unwrap_or(0);
                let mut end = before.unwrap_or(doc_count);

                if let Some(first) = first {
                    end = (start + first).min(end);
                }
                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last };
                }
                let limit = (end - start) as i64;

                let cursor = database2(ctx)?
                    .list_intern_merch(start as i64, limit)
                    .await?;

                let mut connection = Connection::new(start > 0, end < doc_count);
                connection
                    .append_stream(cursor.enumerate().map(|(n, doc)| {
                        let merch = from_document::<InternMerchandise>(doc.unwrap()).unwrap();
                        Edge::with_additional_fields(n + start, merch, EmptyFields)
                    }))
                    .await;
                Ok(connection)
            },
        )
        .await
    }

    pub async fn count_intern_merch(&self, ctx: &Context<'_>) -> Result<usize> {
        Ok(database2(ctx)?.count_intern_merch().await?)
    }
}

#[derive(Default)]
pub struct InternMerchandiseMutation;

#[Object]
impl InternMerchandiseMutation {
    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::MerchandiseModerator")
    )))]
    async fn new_intern_merch(
        &self,
        ctx: &Context<'_>,
        new: NewInternMerchandise,
    ) -> Result<InternMerchandise> {
        let _ = Claim::from_ctx(ctx)?;
        let new_merch = DBInternMerchandise::new(new);
        Ok(database2(ctx)?.new_intern_merch(new_merch.clone()).await?)
    }

    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::MerchandiseModerator")
    )))]
    async fn update_intern_merch(
        &self,
        ctx: &Context<'_>,
        id: InternMerchandiseId,
        update: InternMerchandiseUpdate,
    ) -> Result<InternMerchandise> {
        let _ = Claim::from_ctx(ctx)?;
        Ok(database2(ctx)?.update_intern_merch(id, update).await?)
    }

    async fn change_status(
        &self,
        ctx: &Context<'_>,
        id: InternMerchandiseId,
        _new_status: InternMerchandiseStatus,
    ) -> Result<InternMerchandise> {
        let _ = Claim::from_ctx(ctx)?;
        let merch = database2(ctx)?.get_intern_merch_by_id(id).await?;

        //let orderer_id = merch.orderer.get_id().clone();
        //let _user = database2(ctx)?.get_user_by_id(orderer_id).await?;

        // TODO: fix
        //merch.change_status(new_status, user);

        Ok(merch)
    }

    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::MerchandiseModerator")
    )))]
    async fn delete_intern_merch(
        &self,
        ctx: &Context<'_>,
        id: InternMerchandiseId,
    ) -> Result<bool> {
        let _ = Claim::from_ctx(ctx)?;
        // TODO: check if other collections or documents still have a object refercnce to this project
        // if not we can safely remove the project
        // if there are still refercnces return an error
        let _ = database2(ctx)?.delete_intern_merch(id).await?;

        Ok(true)
    }
}
