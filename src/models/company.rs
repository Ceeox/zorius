use juniper::GraphQLEnum;
use serde::{Deserialize, Serialize};

#[derive(GraphQLEnum, Deserialize, Serialize, Debug)]
pub enum CompanyType {
    GmbH,
    AG,
}

pub struct Company {
    pub id: String,
    pub name: String,
    // pub _type: CompanyType,
    pub street: String,
    pub city: String,
    pub postage_number: String,
}
