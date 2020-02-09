use std::sync::{Arc, Mutex};

use bson::to_bson;
use bson::{bson, doc};
use juniper::DefaultScalarValue;
use juniper::FieldResult;
use juniper::GraphQLObject;
use juniper::RootNode;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};

static MONGO_DB_NAME: &str = "zorius";
static MONGO_DB_COLLECTION_NAME_INTERN: &str = "merchandise_intern";

#[derive(Clone)]
pub struct Context {
    pub client: mongodb::Client,
}

impl juniper::Context for Context {}

#[derive(GraphQLObject, Deserialize, Serialize)]
pub struct InternMerchandise {
    id: String,
    merchandise_name: String,
    count: i32,
    orderer: String,
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

#[derive(juniper::GraphQLEnum, Deserialize, Serialize)]
pub enum InternMerchandiseStatus {
    Ordered,
    Delivered,
    Stored,
    Used,
}

#[derive(juniper::GraphQLInputObject, Deserialize, Serialize)]
#[graphql(description = "Stores internal merchandise infos")]
pub struct NewInternOrder {
    merchandise_name: String,
    count: i32,
    url: Option<String>,
    orderer: String,
    purchased_on: String,
    article_number: Option<String>,
    cost: f64,
    postage: Option<f64>,
    use_case: Option<String>,
}

pub struct InternMerchandiseQueryRoot;

#[juniper::object(Context = Context)]
impl InternMerchandiseQueryRoot {
    fn table_data(ctx: &Context) -> FieldResult<InternMerchandise> {
        Ok(InternMerchandise {
            id: "909fgdg",
            merchandise_name: "Test".to_owned(),
            count: 42,
            orderer: "mw".to_owned(),
            purchased_on: "01/01/2020".to_owned(),
            cost: 6.54,
            status: InternMerchandiseStatus::Ordered,

            merchandise_id: None,
            article_number: None,
            postage: None,
            serial_number: None,
            invoice_number: None,
            use_case: None,
            arived_on: None,
            url: None,
        })
    }
}

pub struct InternMerchandiseMutationRoot;

#[juniper::object(Context = Context)]
impl InternMerchandiseMutationRoot {
    fn new_intern_order(
        ctx: &Context,
        new_intern_order: NewInternOrder,
    ) -> FieldResult<InternMerchandise> {
        let order = InternMerchandise {
            id: "kdas",
            merchandise_name: new_intern_order.merchandise_name,
            count: new_intern_order.count,
            orderer: new_intern_order.orderer,
            purchased_on: new_intern_order.purchased_on,
            cost: new_intern_order.cost,
            status: InternMerchandiseStatus::Ordered,

            merchandise_id: None,
            article_number: new_intern_order.article_number,
            postage: new_intern_order.postage,
            serial_number: None,
            invoice_number: None,
            use_case: new_intern_order.use_case,
            arived_on: None,
            url: new_intern_order.url,
        };
        let db = ctx.client.database(MONGO_DB_NAME);
        let collection = db.collection(MONGO_DB_COLLECTION_NAME_INTERN);
        let doc = bson::to_bson(&order).unwrap();
        collection.insert_one(doc.as_document().unwrap().clone(), None)?;
        Ok(order)
    }
}

pub type Schema = RootNode<'static, InternMerchandiseQueryRoot, InternMerchandiseMutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(
        InternMerchandiseQueryRoot {},
        InternMerchandiseMutationRoot {},
    )
}
