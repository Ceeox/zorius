use bson::de::from_document;
use bson::{doc, from_bson, to_bson};
use chrono::Utc;
use futures::stream::StreamExt;
use juniper::FieldResult;
use uuid::Uuid;

use crate::models::merchandise::intern_merchandise::{
    InternMerchandise, InternMerchandiseStatus, NewInternOrder,
};

use crate::Context;

static MONGO_DB_COLLECTION_NAME_INTERN: &str = "merchandise_intern";

pub struct InternMerchandiseQuery;

impl InternMerchandiseQuery {
    pub async fn table_data(ctx: &Context) -> FieldResult<Vec<InternMerchandise>> {
        let collection = ctx.db.collection(MONGO_DB_COLLECTION_NAME_INTERN);
        // TODO: Limit Query size
        let cursor = collection.find(None, None).await?;
        let res = cursor
            .filter_map(|x| async move {
                println!("{:?}", x);
                let doc = x.unwrap();
                match from_document(doc) {
                    Ok(r) => Some(r),
                    Err(e) => None,
                }
            })
            .collect::<Vec<_>>()
            .await;
        Ok(res)
    }

    pub async fn get_order(
        ctx: &Context,
        order_id: String,
    ) -> FieldResult<Option<InternMerchandise>> {
        let collection = ctx.db.collection(MONGO_DB_COLLECTION_NAME_INTERN);
        let filter = doc! { "_id": order_id };
        let item = match collection.find_one(Some(filter), None).await? {
            None => return Ok(None),
            Some(r) => to_bson(&r)?,
        };
        let res = from_bson::<InternMerchandise>(item)?;
        println!("{:?}", res);
        Ok(Some(res))
    }
}

pub struct InternMerchandiseMutation;

impl InternMerchandiseMutation {
    pub async fn new_intern_order(
        ctx: &Context,
        new_intern_order: NewInternOrder,
    ) -> FieldResult<InternMerchandise> {
        let order = InternMerchandise {
            id: Uuid::new_v4().to_string(),
            merchandise_name: new_intern_order.merchandise_name,
            bought_through: None,
            count: new_intern_order.count,
            orderer: new_intern_order.orderer,
            purchased_on: Utc::now(),
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
        };
        let collection = ctx.db.collection("merchandise_intern");
        let bson = bson::to_bson(&order)?;
        let doc = bson
            .as_document()
            .expect("Failed to convert to bson::Document");
        let _ = collection.insert_one(doc.clone(), None).await?;
        Ok(order)
    }

    // TODO: implement an update funtion
    pub fn update_intern_order(
        ctx: &Context,
        new_intern_order: NewInternOrder,
    ) -> FieldResult<String> {
        unimplemented!();
    }
}
