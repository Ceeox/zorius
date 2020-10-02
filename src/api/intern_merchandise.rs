use bson::{bson, doc, from_bson, to_bson};
use bson::{de::from_document, oid::ObjectId, to_document, Bson, DateTime};
use chrono::Utc;
use futures::stream::StreamExt;
use juniper::{graphql_value, EmptySubscription, FieldError, FieldResult, RootNode};
use mongodb::options::FindOptions;

use crate::models::merchandise::intern_merchandise::{
    InternMerchandise, InternMerchandiseList, InternMerchandiseStatus, InternMerchandiseUpdate,
    NewInternOrder,
};

use crate::Context;

static MONGO_DB_COLLECTION_NAME_INTERN: &str = "merchandise_intern";

const MAX_TABLE_DATA_RESULTS: i64 = 50;

pub struct InternMerchandiseQuery;

impl InternMerchandiseQuery {
    pub async fn table_data(ctx: &Context) -> FieldResult<InternMerchandiseList> {
        let collection = ctx.db.collection(MONGO_DB_COLLECTION_NAME_INTERN);
        // TODO: Limit Query size
        let find_opt = Some(FindOptions::builder().limit(MAX_TABLE_DATA_RESULTS).build());
        let cursor = collection.find(None, find_opt).await?;
        let res = cursor
            .filter_map(|x| async move {
                match from_document(x.clone().unwrap()) {
                    Ok(r) => Some(r),
                    Err(e) => {
                        eprintln!("Got error on {:?} with error: {:?}", x, e);
                        None
                    }
                }
            })
            .collect::<Vec<_>>()
            .await;
        let res = InternMerchandiseList { intern_list: res };
        Ok(res)
    }

    pub async fn get_order(ctx: &Context, order_id: ObjectId) -> FieldResult<InternMerchandise> {
        let collection = ctx.db.collection(MONGO_DB_COLLECTION_NAME_INTERN);
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
}

pub struct InternMerchandiseMutation;

impl InternMerchandiseMutation {
    pub async fn new_intern_order(
        ctx: &Context,
        new_intern_order: NewInternOrder,
    ) -> FieldResult<InternMerchandise> {
        let order = InternMerchandise {
            id: ObjectId::new(),
            merchandise_name: new_intern_order.merchandise_name,
            // bought_through: None,
            count: new_intern_order.count,
            orderer: new_intern_order.orderer,
            purchased_on: Utc::now().into(),
            cost: new_intern_order.cost,
            status: InternMerchandiseStatus::Ordered,
            url: new_intern_order.url,
            use_case: new_intern_order.use_case,
            article_number: new_intern_order.article_number,
            postage: new_intern_order.postage,
            project_leader: Some(new_intern_order.project_leader),
            location: Some(new_intern_order.location),
            shop: Some(new_intern_order.shop),

            merchandise_id: None,
            serial_number: None,
            arived_on: None,
            invoice_number: None,
            created_date: Utc::now().into(),
            updated_date: Utc::now().into(),
        };
        let collection = ctx.db.collection("merchandise_intern");
        let doc = to_document(&order)?;
        let _ = collection.insert_one(doc.clone(), None).await?;
        Ok(order)
    }

    // TODO: implement an update funtion
    pub async fn update_intern_order(
        ctx: &Context,
        order_id: ObjectId,
        update: InternMerchandiseUpdate,
    ) -> FieldResult<InternMerchandise> {
        let collection = ctx.db.collection("merchandise_intern");
        let mut order = InternMerchandiseQuery::get_order(ctx, order_id.clone()).await?;
        order.update(update);
        let query = doc! {
            "_id": order_id
        };
        let update_doc = to_document(&order)?;
        let _ = collection.update_one(query, update_doc, None).await?;
        Ok(order)
    }
}
