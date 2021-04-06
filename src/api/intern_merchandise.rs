use async_graphql::{Context, Error, Object, Result};
use bson::doc;
use bson::{de::from_document, oid::ObjectId, to_document};
use futures::stream::{StreamExt, TryStreamExt};
use mongod::{AsFilter, AsUpdate, Client, Comparator, Updates};
use mongodb::options::FindOptions;

use crate::{
    helper::Update,
    models::merchandise::intern_merchandise::{
        InternMerchandise, InternMerchandiseFilter, InternMerchandiseUpdate, NewMerchandiseIntern,
    },
};

use super::{claim::Claim, database, MDB_COLL_INTERN_MERCH};

// replace with gql query
const MAX_TABLE_DATA_RESULTS: i64 = 50;

#[derive(Default)]
pub struct InternMerchandiseQuery;

#[Object]
impl InternMerchandiseQuery {
    async fn table_data(&self, ctx: &Context<'_>) -> Result<Vec<InternMerchandise>> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_INTERN_MERCH);
        let find_opt = Some(FindOptions::builder().limit(50).build());
        let cursor = collection.find(None, find_opt).await?;
        let res = cursor
            .filter_map(|doc| async move {
                match doc {
                    Err(_) => None,
                    Ok(r) => Some(from_document::<InternMerchandise>(r)),
                }
            })
            .try_collect::<Vec<_>>()
            .await?;

        Ok(res)
    }

    async fn get_order(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> Result<Option<InternMerchandise>> {
        let _ = Claim::from_ctx(ctx)?;
        let client = ctx.data::<Client>()?;
        let res = client
            .find_one::<InternMerchandise, _>(id.into_filter())
            .await?;
        Ok(res)
    }
}

#[derive(Default)]
pub struct InternMerchandiseMutation;

#[Object]
impl InternMerchandiseMutation {
    async fn new_merchandise_intern(
        &self,
        ctx: &Context<'_>,
        new_intern_merch: NewMerchandiseIntern,
    ) -> Result<InternMerchandise> {
        let _ = Claim::from_ctx(ctx)?;
        let new_merch = InternMerchandise::new(new_intern_merch);
        let client = ctx.data::<Client>()?;
        let _ = client.insert_one(new_merch.clone());
        Ok(new_merch)
    }

    async fn update_merchandise_intern(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
        update: InternMerchandiseUpdate,
    ) -> Result<Option<InternMerchandise>> {
        let _ = Claim::from_ctx(ctx)?;
        let client = ctx.data::<Client>()?;
        let update = Updates {
            set: Some(update),
            ..Updates::default()
        };
        let _ = client
            .update_one::<InternMerchandise, _, _>(id.clone().into_filter(), update)
            .await?;
        let updated = client
            .find_one::<InternMerchandise, _>(id.into_filter())
            .await?;
        Ok(updated)
    }

    async fn delete_merchandise_intern(&self, ctx: &Context<'_>, id: ObjectId) -> Result<bool> {
        let _ = Claim::from_ctx(ctx)?;
        let client = ctx.data::<Client>()?;
        let filter = id.clone().into_filter();
        Ok(client.delete_one::<InternMerchandise, _>(filter).await?)
    }
}
