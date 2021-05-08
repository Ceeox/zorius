use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    guard::Guard,
    Context, Error, Object, Result,
};
use bson::{de::from_document, doc, oid::ObjectId};
use futures::stream::StreamExt;

use crate::{
    api::database2,
    database::MDB_COLL_INTERN_MERCH,
    models::{
        merchandise::intern_merchandise::{
            InternMerchResponse, InternMerchandise, InternMerchandiseId, InternMerchandiseStatus,
            InternMerchandiseUpdate, NewInternMerchandise,
        },
        roles::{Role, RoleGuard},
    },
};

use super::{claim::Claim, database};

#[derive(Default)]
pub struct InternMerchandiseQuery;

#[Object]
impl InternMerchandiseQuery {
    async fn get_intern_merch_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> Result<Option<InternMerchResponse>> {
        let _ = Claim::from_ctx(ctx)?;
        match database2(ctx)?.get_intern_merch_by_id(id).await? {
            Some(r) => Ok(Some(r)),
            None => Err(Error::new("intern merch could not be found")),
        }
    }

    async fn get_intern_merch_by_merch_id(
        &self,
        ctx: &Context<'_>,
        merchandise_id: i32,
    ) -> Result<Option<InternMerchandise>> {
        let _ = Claim::from_ctx(ctx)?;
        match database2(ctx)?
            .get_intern_merch_by_merch_id(merchandise_id)
            .await?
        {
            Some(r) => Ok(Some(r)),
            None => Err(Error::new("intern merch could not be found")),
        }
    }

    async fn list_intern_merch(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, InternMerchResponse, EmptyFields, EmptyFields>> {
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
                        let merch = from_document::<InternMerchResponse>(doc.unwrap()).unwrap();
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
        let new_merch = InternMerchandise::new(new);
        let _ = database2(ctx)?.new_intern_merch(new_merch.clone()).await?;
        Ok(new_merch)
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
    ) -> Result<Option<InternMerchResponse>> {
        let _ = Claim::from_ctx(ctx)?;
        Ok(database2(ctx)?.update_intern_merch(id, update).await?)
    }

    async fn change_status(
        &self,
        ctx: &Context<'_>,
        id: InternMerchandiseId,
        new_status: InternMerchandiseStatus,
    ) -> Result<Option<InternMerchandise>> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_INTERN_MERCH);
        let filter = doc! {"_id": id};
        let mut merch = match collection.find_one(filter, None).await? {
            None => return Ok(None),
            Some(doc) => from_document::<InternMerchandise>(doc)?,
        };

        let orderer_id = merch.orderer.clone();
        let user = match database2(ctx)?.get_user_by_id(orderer_id).await? {
            None => return Err(Error::new("the orderer could not be found")),
            Some(r) => r,
        };

        merch.change_status(new_status, user);

        Ok(None)
    }
}
