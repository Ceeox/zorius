use async_graphql::{Context, Object};
use bson::doc;
use bson::{de::from_document, oid::ObjectId, to_document};
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::options::FindOptions;

use crate::{
    errors::ZoriusError,
    models::merchandise::intern_merchandise::{
        InternMerchandise, InternMerchandiseResponse, NewInternMerchandiseQuery,
        UpdateInternMerchandiseQuery,
    },
};



// replace with gql query
const MAX_TABLE_DATA_RESULTS: i64 = 50;

pub struct InternMerchandiseQuery;

#[Object]
impl InternMerchandiseQuery {
    pub async fn table_data(
        ctx: &Context<'_>,
    ) -> Result<&'_ Vec<InternMerchandiseResponse>, ZoriusError> {
        let collection = ctx.db.collection(MDB_COLL_NAME_INTERN);
        let find_opt = Some(FindOptions::builder().limit(MAX_TABLE_DATA_RESULTS).build());
        let cursor = collection.find(None, find_opt).await?;
        let res = cursor
            .filter_map(|doc| async move {
                match doc {
                    Err(_) => None,
                    Ok(r) => Some(from_document::<InternMerchandiseResponse>(r)),
                }
            })
            .try_collect::<Vec<InternMerchandiseResponse>>()
            .await?;

        Ok(res)
    }
    /*
    pub async fn get_order(
        &self,
        ctx: &Context,
        order_id: ObjectId,
    ) -> FieldResult<InternMerchandiseResponse> {
        let collection = ctx.db.collection(MDB_COLL_NAME_INTERN);
        let filter = doc! { "_id": order_id };
        match collection.find_one(Some(filter), None).await? {
            None => {
                return Err(FieldError::new(
                    "specified order not found",
                    graphql_value!({ "error": "specified order not found" }),
                ))
            }
            Some(r) => Ok(from_document(r)?),
        }
    }
    */
}

pub struct InternMerchandiseMutation;

impl InternMerchandiseMutation {
    /*
    pub async fn new_intern_order(
        ctx: &Context,
        new_intern_merchandise: NewInternMerchandiseQuery,
    ) -> FieldResult<InternMerchandiseResponse> {
        let order = InternMerchandise::new(new_intern_merchandise);
        let collection = ctx.db.collection(MDB_COLL_NAME_INTERN);
        let doc = to_document(&order)?;
        let _ = collection.insert_one(doc.clone(), None).await?;
        Ok(order.into())
    }

    pub async fn update_intern_order(
        ctx: &Context,
        order_id: ObjectId,
        update: UpdateInternMerchandiseQuery,
    ) -> FieldResult<InternMerchandiseResponse> {
        let collection = ctx.db.collection(MDB_COLL_NAME_INTERN);
        let filter = doc! { "_id": order_id.clone() };
        let mut order: InternMerchandise = match collection.find_one(Some(filter), None).await? {
            None => {
                return Err(FieldError::new(
                    "specified order not found",
                    graphql_value!({ "error": "specified order not found" }),
                ))
            }
            Some(r) => from_document(r)?,
        };
        order.update(update);
        let query = doc! {
            "_id": order_id
        };

        let update_doc = to_document(&order)?;
        let _ = collection.update_one(query, update_doc, None).await?;
        Ok(order.into())
    }
    */
}
