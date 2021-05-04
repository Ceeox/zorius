use std::collections::HashMap;

use async_graphql::{Error, Result};
use bson::{doc, from_document, to_document};
use futures::StreamExt;
use mongodb::{
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Client, Database as MongoDB,
};

use crate::models::{
    merchandise::intern_merchandise::{
        InternMerchResponse, InternMerchandise, InternMerchandiseId,
    },
    user::{NewUser, User, UserId, UserUpdate},
    work_report::{WorkReport, WorkReportId, WorkReportResponse, WorkReportUpdate},
};

pub(crate) static MDB_COLL_NAME_USERS: &str = "users";
pub(crate) static MDB_COLL_WORK_ACCOUNTS: &str = "workaccounts";
pub(crate) static MDB_COLL_WORK_REPORTS: &str = "work_reports";
pub(crate) static MDB_COLL_INTERN_MERCH: &str = "merchandise_intern";
pub(crate) static MDB_COLL_ROLES: &str = "roles";

pub struct Database {
    _client: Client,
    database: MongoDB,
}

impl Database {
    pub fn new(client: Client, database: mongodb::Database) -> Self {
        Self {
            _client: client,
            database,
        }
    }

    pub async fn get_user_by_id(&self, id: UserId) -> Result<Option<User>> {
        let collection = self.database.collection(MDB_COLL_NAME_USERS);
        let filter = doc! {"_id": id};
        let user = match collection.find_one(filter, None).await? {
            None => return Ok(None),
            Some(doc) => from_document::<User>(doc)?,
        };
        Ok(Some(user))
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<Option<User>> {
        let col = self.database.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "email": email.clone() };
        let user = match col.find_one(filter, None).await? {
            None => return Ok(None),
            Some(doc) => from_document::<User>(doc)?,
        };
        Ok(Some(user))
    }

    pub async fn new_user(&self, new_user: NewUser) -> Result<User> {
        let user = User::new(new_user);
        let col = self.database.collection(MDB_COLL_NAME_USERS);
        let doc = to_document(&user)?;
        let _ = col.insert_one(doc.clone(), None).await?;
        Ok(user)
    }

    pub async fn update_user(&self, id: UserId, user_update: UserUpdate) -> Result<User> {
        let col = self.database.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "_id": id };

        let mut update = bson::Document::new();
        update.insert("$set", bson::to_bson(&user_update)?);

        let options = FindOneAndUpdateOptions::builder()
            .return_document(Some(ReturnDocument::After))
            .build();

        let user = match col
            .find_one_and_update(filter, update, Some(options))
            .await?
        {
            None => return Err(Error::new("specified user not found")),
            Some(r) => r,
        };
        Ok(from_document(user)?)
    }

    pub async fn get_intern_merch_by_id(
        &self,
        id: InternMerchandiseId,
    ) -> Result<Option<InternMerchResponse>> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        let pipeline = vec![
            doc! {"$match": {"_id": id}},
            doc! {"$lookup": {
                    "from": MDB_COLL_NAME_USERS,
                    "localField": "orderer",
                    "foreignField": "_id",
                    "as": "orderer"
                }
            },
            doc! {
                "$unwind": {
                    "path": "$orderer"
                }
            },
        ];
        let mut doc = collection.aggregate(pipeline, None).await?;
        match doc.next().await {
            Some(r) => Ok(Some(from_document(r?)?)),
            None => Ok(None),
        }
    }

    pub async fn get_intern_merch_by_merch_id(
        &self,
        merchandise_id: i32,
    ) -> Result<Option<InternMerchandise>> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        let filter = doc! {"merchandise_id": merchandise_id};
        match collection.find_one(filter, None).await? {
            None => Ok(None),
            Some(doc) => Ok(Some(from_document::<InternMerchandise>(doc)?)),
        }
    }

    pub async fn count_intern_merch(&self) -> Result<usize> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        Ok(collection.estimated_document_count(None).await? as usize)
    }

    pub async fn get_work_report_by_id(
        &self,
        id: WorkReportId,
        user_id: UserId,
    ) -> Result<Option<WorkReportResponse>> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        let pipeline = vec![
            doc! {"$match": {
                    "_id": id,
                    "user_id": user_id
            }},
            doc! {"$lookup": {
                    "from": MDB_COLL_NAME_USERS,
                    "localField": "user_id",
                    "foreignField": "_id",
                    "as": "user"
                }
            },
            doc! {"$lookup": {
                    "from": MDB_COLL_WORK_REPORTS,
                    "localField": "project_id",
                    "foreignField": "_id",
                    "as": "project"
                }
            },
            doc! {"$unwind": {
                    "path": "$user_id",
                    "path": "$project_id"
                }
            },
        ];
        let mut doc = collection.aggregate(pipeline, None).await?;
        match doc.next().await {
            Some(r) => Ok(Some(from_document(r?)?)),
            None => Ok(None),
        }
    }

    pub async fn update_work_report(
        &self,
        id: WorkReportId,
        update: WorkReportUpdate,
    ) -> Result<WorkReport> {
        let col = self.database.collection(MDB_COLL_WORK_REPORTS);
        let filter = doc! { "_id": id };

        let mut update = bson::Document::new();
        update.insert("$set", bson::to_bson(&update)?);

        let options = FindOneAndUpdateOptions::builder()
            .return_document(Some(ReturnDocument::After))
            .build();

        let workreport = match col
            .find_one_and_update(filter, update, Some(options))
            .await?
        {
            None => return Err(Error::new("specified work_report not found")),
            Some(r) => r,
        };
        Ok(from_document(workreport)?)
    }
}
