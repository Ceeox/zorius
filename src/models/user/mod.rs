use bson::oid::ObjectId;
use bson::DateTime;
use chrono::Utc;
use juniper::{GraphQLInputObject, GraphQLObject};
use pwhash::sha512_crypt;
use serde::{Deserialize, Serialize};

use super::permissions::{Permissions, UserPermissionUpdate};

#[derive(GraphQLObject, Deserialize, Serialize, Debug)]
#[graphql(description = "Stores the userdata")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub email: String,
    pub password_hash: Option<String>,

    pub username: String,
    pub created_at: DateTime,
    pub invitation_pending: bool,

    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub last_updated: Option<DateTime>,

    pub permissions: Permissions,

    pub deleted: bool,
}

impl User {
    pub fn new(new_user: NewUser) -> Self {
        let mut user = Self {
            id: ObjectId::new(),
            email: new_user.email,
            password_hash: None,
            username: new_user.username,
            firstname: new_user.firstname,
            lastname: new_user.lastname,
            created_at: Utc::now().into(),
            invitation_pending: true,
            deleted: false,
            last_updated: Some(Utc::now().into()),
            permissions: Permissions::default(),
        };
        user.hash_password(&new_user.password);
        user
    }

    pub fn hash_password(&mut self, password: &str) {
        self.password_hash = Some(
            sha512_crypt::hash(password.as_bytes())
                .expect("ERROR: system random number generator cannot be opened!"),
        );
    }

    pub fn check_password(&mut self, password: &str) -> bool {
        match self.password_hash {
            None => false,
            Some(ref r) => sha512_crypt::verify(password.as_bytes(), r),
        }
    }

    pub fn update(&mut self, update: UserUpdate) {
        self.email = update.email.unwrap_or(self.email.clone());
        match update.password {
            Some(r) => self.hash_password(&r),
            None => {}
        }
        self.username = update.username.unwrap_or(self.username.clone());
        self.firstname = Some(
            update
                .firstname
                .unwrap_or(self.firstname.clone().unwrap_or_default()),
        );
        self.lastname = Some(
            update
                .lastname
                .unwrap_or(self.lastname.clone().unwrap_or_default()),
        );
        self.deleted = update.deleted.unwrap_or(self.deleted);
        self.last_updated = Some(Utc::now().into());
        match update.permissions {
            None => self.permissions = Permissions::default(),
            Some(r) => self.permissions.user_update(r),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, GraphQLInputObject)]
#[graphql(description = "new user data, used to insert to database")]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, GraphQLInputObject)]
#[graphql(description = "udpate the user data")]
pub struct UserUpdate {
    pub email: Option<String>,
    pub password: Option<String>,
    pub username: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub deleted: Option<bool>,
    pub permissions: Option<UserPermissionUpdate>,
}
