use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(juniper::GraphQLInputObject, Deserialize, Serialize, Debug)]
#[graphql(description = "Stores the userdata")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub email: String,
    pub password_hash: Option<String>,

    pub user_name: String,
    pub created_at: DateTime<Utc>,
    pub invitation_pending: bool,

    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub last_updated: Option<DateTime<Utc>>,

    pub deleted: bool,
}

#[derive(Deserialize, Serialize, Debug, juniper::GraphQLInputObject)]
#[graphql(description = "new user data, used to insert to database")]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub username: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}
