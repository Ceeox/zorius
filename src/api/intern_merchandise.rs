use async_graphql::{Context, Error, Object, Result};
use bson::doc;
use bson::{de::from_document, oid::ObjectId, to_document};
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::options::FindOptions;

use crate::models::merchandise::intern_merchandise::{MerchandiseIntern, NewMerchandiseIntern};

use super::{claim::Claim, database, MDB_COLL_INTERN_MERCH};

// replace with gql query
const MAX_TABLE_DATA_RESULTS: i64 = 50;

#[derive(Default)]
pub struct InternMerchandiseQuery;

#[Object]
impl InternMerchandiseQuery {
    async fn table_data(&self, ctx: &Context<'_>) -> Result<Vec<MerchandiseIntern>> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_INTERN_MERCH);
        let find_opt = Some(FindOptions::builder().limit(50).build());
        let cursor = collection.find(None, find_opt).await?;
        let res = cursor
            .filter_map(|doc| async move {
                match doc {
                    Err(_) => None,
                    Ok(r) => Some(from_document::<MerchandiseIntern>(r)),
                }
            })
            .try_collect::<Vec<_>>()
            .await?;

        Ok(res)
    }

    async fn get_order(&self, ctx: &Context<'_>, id: ObjectId) -> Result<MerchandiseIntern> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_INTERN_MERCH);
        let filter = doc! { "_id": id };
        match collection.find_one(Some(filter), None).await? {
            None => return Err(Error::new("intern order not found")),
            Some(r) => Ok(from_document(r)?),
        }
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
    ) -> Result<MerchandiseIntern> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_INTERN_MERCH);

        let new_merch_intern = MerchandiseIntern::new(new_intern_merch);
        let im_id = new_merch_intern.get_id().clone();
        let insert = to_document(&new_merch_intern)?;
        let _ = collection.insert_one(insert, None).await?;

        let filter = doc! { "_id": im_id };
        let wa = collection.find_one(filter, None).await?.unwrap();
        Ok(from_document(wa)?)
    }
}
