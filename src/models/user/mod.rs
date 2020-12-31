use async_graphql::SimpleObject;
use bson::oid::ObjectId;
use bson::DateTime;
use chrono::Utc;
use pwhash::sha512_crypt;
use serde::{Deserialize, Serialize};

pub type UserId = ObjectId;

#[derive(Deserialize, Debug)]
pub struct NewUserQuery {
    pub email: String,
    pub username: String,
    pub password: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, SimpleObject)]
pub struct User {
    #[serde(rename = "_id")]
    id: UserId,
    email: String,
    #[graphql(skip)]
    password_hash: String,
    username: String,
    created_at: DateTime,
    invitation_pending: bool,
    firstname: Option<String>,
    lastname: Option<String>,
    last_updated: Option<DateTime>,
    deleted: bool,
    //    claim: Option<Claim>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Claim {
    pub sub: String,
    pub exp: usize,
}

impl User {
    pub fn new(
        email: String,
        username: String,
        password: String,
        firstname: Option<String>,
        lastname: Option<String>,
    ) -> Self {
        let password_hash = User::hash_password(&password);
        Self {
            id: ObjectId::new(),
            email,
            password_hash,
            username,
            firstname,
            lastname,
            created_at: Utc::now().into(),
            invitation_pending: true,
            deleted: false,
            last_updated: Some(Utc::now().into()),
        }
    }

    pub fn hash_password(password: &str) -> String {
        sha512_crypt::hash(password.as_bytes())
            .expect("system random number generator cannot be opened!")
    }

    pub fn is_password_correct(&self, password: &str) -> bool {
        sha512_crypt::verify(password.as_bytes(), &self.password_hash)
    }
    pub fn is_deleted(&self) -> bool {
        self.deleted
    }
}
