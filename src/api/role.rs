use std::future;

use bson::{doc, oid::ObjectId, to_document};
use futures::{stream::StreamExt, TryStreamExt};
use juniper::{graphql_value, FieldError, FieldResult};
use mongodb::Cursor;
use mongodb::{bson::from_document, options::FindOptions};

use crate::models::user::{NewUserQuery, UpdateUserQuery, User, UserResponse};
use crate::Context;

static MDB_COLL_NAME_USERS: &str = "roles";
static MAX_USER_QUERY: usize = 50;

pub struct RoleQuery;

impl RoleQuery {
    pub async fn list_role(ctx: &Context) -> FieldResult<Vec<UserResponse>> {
        unimplemented!()
    }
}

pub struct RoleMutation;

impl RoleMutation {
    pub async fn new_role(ctx: &Context, new_user: NewUserQuery) -> FieldResult<UserResponse> {
        unimplemented!()
    }
}
