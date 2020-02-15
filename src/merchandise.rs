use std::sync::Mutex;

use bson::{bson, doc, from_bson, to_bson, Bson};
use juniper::{FieldResult, GraphQLObject, RootNode};
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

static MONGO_DB_NAME: &str = "zorius";
static MONGO_DB_COLLECTION_NAME_INTERN: &str = "merchandise_intern";

#[derive(Clone)]
pub struct Context {
    pub client: mongodb::Client,
}

impl juniper::Context for Context {}
#[derive(GraphQLObject, Deserialize, Serialize, Debug)]
pub struct InternMerchandise {
    _id: String,
    merchandise_name: String,
    count: i32,
    orderer_id: String,
    purchased_on: String,
    cost: f64,
    status: InternMerchandiseStatus,

    merchandise_id: Option<i32>,
    article_number: Option<String>,
    postage: Option<f64>,
    serial_number: Option<String>,
    invoice_number: Option<i32>,
    use_case: Option<String>,
    arived_on: Option<String>,
    url: Option<String>,
}

#[derive(juniper::GraphQLEnum, Deserialize, Serialize, Debug)]
pub enum InternMerchandiseStatus {
    Ordered,
    Arrived,
    Stored,
    Used,
}

#[derive(juniper::GraphQLInputObject, Deserialize, Serialize)]
#[graphql(description = "Stores internal merchandise infos")]
pub struct NewInternOrder {
    merchandise_name: String,
    count: i32,
    url: Option<String>,
    orderer_id: String,
    purchased_on: String,
    article_number: Option<String>,
    cost: f64,
    postage: Option<f64>,
    use_case: Option<String>,
}

pub struct InternMerchandiseQueryRoot;

#[juniper::object(Context = Context)]
impl InternMerchandiseQueryRoot {
    fn table_data(ctx: &Context) -> FieldResult<Vec<InternMerchandise>> {
        let db = ctx.client.database(MONGO_DB_NAME);
        let collection = db.collection(MONGO_DB_COLLECTION_NAME_INTERN);
        // TODO: Limit Query size
        let cursor = collection.find(None, None)?;
        Ok(cursor
            .map(|x| {
                let bson = to_bson(&x.unwrap()).unwrap();
                from_bson::<InternMerchandise>(bson).unwrap()
            })
            .collect())
    }
    fn get_order(ctx: &Context, id: String) -> FieldResult<Option<InternMerchandise>> {
        let db = ctx.client.database(MONGO_DB_NAME);
        let collection = db.collection(MONGO_DB_COLLECTION_NAME_INTERN);
        let filter = doc! { "_id": id};
        let item = match collection.find_one(filter, None)? {
            None => return Ok(None),
            Some(r) => to_bson(&r)?,
        };
        let res = from_bson::<InternMerchandise>(item)?;
        println!("{:?}", res);
        Ok(Some(res))
    }
}

pub struct InternMerchandiseMutationRoot;

#[juniper::object(Context = Context)]
impl InternMerchandiseMutationRoot {
    fn new_intern_order(
        ctx: &Context,
        new_intern_order: NewInternOrder,
    ) -> FieldResult<InternMerchandise> {
        let mut order = InternMerchandise {
            _id: Uuid::new_v4().to_string(),
            merchandise_name: new_intern_order.merchandise_name,
            count: new_intern_order.count,
            orderer_id: new_intern_order.orderer_id,
            purchased_on: new_intern_order.purchased_on,
            cost: new_intern_order.cost,
            status: InternMerchandiseStatus::Ordered,
            url: new_intern_order.url,
            use_case: new_intern_order.use_case,
            article_number: new_intern_order.article_number,
            postage: new_intern_order.postage,

            merchandise_id: None,
            serial_number: None,
            invoice_number: None,
            arived_on: None,
        };
        let db = ctx.client.database(MONGO_DB_NAME);
        let collection = db.collection(MONGO_DB_COLLECTION_NAME_INTERN);
        let bson: Bson = bson::to_bson(&order)?;
        let doc = bson
            .as_document()
            .expect("Failed to convert to bson::Document");
        let _ = collection.insert_one(doc.clone(), None)?;
        Ok(order)
    }

    // TODO: implement an update funtion
    fn update_intern_order(ctx: &Context, new_intern_order: NewInternOrder) -> FieldResult<String> {
        Ok(String::new())
    }
}

pub type Schema = RootNode<'static, InternMerchandiseQueryRoot, InternMerchandiseMutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(
        InternMerchandiseQueryRoot {},
        InternMerchandiseMutationRoot {},
    )
}
