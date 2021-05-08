use async_graphql::{Error, Result};
use bson::{doc, from_document, to_document, Document};
use futures::StreamExt;
use mongodb::{
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Client, Cursor, Database as MongoDB,
};

use crate::{
    helper::AggregateBuilder,
    models::{
        merchandise::intern_merchandise::{
            InternMerchResponse, InternMerchandise, InternMerchandiseId, InternMerchandiseUpdate,
        },
        user::{NewUser, User, UserId, UserUpdate},
        work_report::{
            customer::{Customer, CustomerId, CustomerResponse, CustomerUpdate, NewCustomer},
            NewWorkReport, WorkReport, WorkReportId, WorkReportResponse, WorkReportUpdate,
        },
    },
};

pub(crate) static MDB_COLL_NAME_USERS: &str = "users";
pub(crate) static MDB_COLL_WORK_REPORTS: &str = "work_reports";
pub(crate) static MDB_COLL_INTERN_MERCH: &str = "merchandise_intern";
pub(crate) static MDB_COLL_CUSTOMERS: &str = "customers";

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

    pub async fn list_users(&self, start: i64, limit: i64) -> Result<Cursor<Document>> {
        let collection = self.database.collection(MDB_COLL_NAME_USERS);
        let options = FindOptions::builder()
            .skip(start as i64)
            .limit(limit)
            .build();
        Ok(collection.find(None, options).await?)
    }

    pub async fn count_users(&self) -> Result<usize> {
        let collection = self.database.collection(MDB_COLL_NAME_USERS);
        Ok(collection.estimated_document_count(None).await? as usize)
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

    pub async fn reset_password(&self, user_id: UserId, password_hash: &str) -> Result<()> {
        let update = doc! {"$set" : doc! {"password_hash": password_hash }};
        let filter = doc! {"_id": user_id};
        let collection = self.database.collection(MDB_COLL_NAME_USERS);
        let _ = collection.update_one(filter, update, None).await?;
        Ok(())
    }

    pub async fn get_intern_merch_by_id(
        &self,
        id: InternMerchandiseId,
    ) -> Result<Option<InternMerchResponse>> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        let pipeline = AggregateBuilder::new()
            .matching(("_id", id))
            .lookup(MDB_COLL_NAME_USERS, "orderer", "_id", "orderer")
            .unwind("$orderer", None, None)
            .build();
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

    pub async fn list_intern_merch(&self, start: i64, limit: i64) -> Result<Cursor<Document>> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        let pipeline = AggregateBuilder::new()
            .skip(start as i64)
            .limit(limit)
            .lookup(MDB_COLL_NAME_USERS, "orderer", "_id", "orderer")
            .unwind("$orderer", None, None)
            .build();
        Ok(collection.aggregate(pipeline, None).await?)
    }

    pub async fn count_intern_merch(&self) -> Result<usize> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        Ok(collection.estimated_document_count(None).await? as usize)
    }

    pub async fn new_intern_merch(&self, new_intern_merch: InternMerchandise) -> Result<()> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        let doc = to_document(&new_intern_merch)?;
        let _ = collection.insert_one(doc, None).await?;
        Ok(())
    }

    pub async fn update_intern_merch(
        &self,
        id: InternMerchandiseId,
        update: InternMerchandiseUpdate,
    ) -> Result<Option<InternMerchResponse>> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        let filter = doc! {"_id": id};
        let update = doc! {"$set": bson::to_bson(&update)?};
        let options = FindOneAndUpdateOptions::builder()
            .return_document(Some(ReturnDocument::After))
            .build();
        match collection
            .find_one_and_update(filter, update, Some(options))
            .await?
        {
            None => Ok(None),
            Some(doc) => Ok(Some(from_document(doc)?)),
        }
    }

    pub async fn get_work_report_by_id(
        &self,
        id: WorkReportId,
        user_id: UserId,
    ) -> Result<Option<WorkReportResponse>> {
        let collection = self.database.collection(MDB_COLL_WORK_REPORTS);
        let pipeline = AggregateBuilder::new()
            .matching(("_id", &id))
            .matching(("user_id", &user_id))
            .lookup(MDB_COLL_NAME_USERS, "user_id", "_id", "user")
            .lookup(MDB_COLL_WORK_REPORTS, "project_id", "_id", "project")
            .lookup(MDB_COLL_WORK_REPORTS, "customer_id", "_id", "customer")
            .unwind("$user", None, None)
            .unwind("$project", None, Some(true))
            .unwind("$customer", None, None)
            .build();
        let mut doc = collection.aggregate(pipeline, None).await?;
        match doc.next().await {
            Some(r) => Ok(Some(from_document(r?)?)),
            None => Ok(None),
        }
    }

    pub async fn list_work_report(
        &self,
        user_id: UserId,
        start: i64,
        limit: i64,
    ) -> Result<Cursor<Document>> {
        let collection = self.database.collection(MDB_COLL_WORK_REPORTS);
        let pipeline = AggregateBuilder::new()
            .skip(start)
            .limit(limit)
            .matching(("user_id", user_id))
            .lookup(MDB_COLL_NAME_USERS, "user_id", "_id", "user")
            .lookup(MDB_COLL_WORK_REPORTS, "project_id", "_id", "project")
            .lookup(MDB_COLL_CUSTOMERS, "customer_id", "_id", "customer")
            .unwind("$user", None, None)
            .unwind("$project", None, Some(true))
            .unwind("$customer", None, None)
            .build();
        Ok(collection.aggregate(pipeline, None).await?)
    }

    pub async fn count_work_reports(&self) -> Result<usize> {
        let collection = self.database.collection(MDB_COLL_WORK_REPORTS);
        Ok(collection.estimated_document_count(None).await? as usize)
    }

    pub async fn new_work_report(
        &self,
        user_id: UserId,
        new: NewWorkReport,
    ) -> Result<WorkReportResponse> {
        let col = self.database.collection(MDB_COLL_WORK_REPORTS);
        let new_work_report = WorkReport::new(user_id.clone(), new);
        let doc = to_document(&new_work_report)?;

        let _ = col.insert_one(doc, None).await?;
        Ok(self
            .get_work_report_by_id(new_work_report.get_id().clone(), user_id)
            .await?
            .unwrap())
    }

    pub async fn update_work_report(
        &self,
        id: WorkReportId,
        user_id: UserId,
        work_report_update: WorkReportUpdate,
    ) -> Result<bool> {
        let col = self.database.collection(MDB_COLL_WORK_REPORTS);
        let filter = doc! { "_id": id , "user_id": user_id };
        let mut update = bson::Document::new();
        update.insert("$set", bson::to_bson(&work_report_update)?);

        let _ = col.update_one(filter, update, None).await?;
        Ok(true)
    }

    pub async fn get_customer_by_id(&self, id: CustomerId) -> Result<Option<CustomerResponse>> {
        let collection = self.database.collection(MDB_COLL_CUSTOMERS);
        let pipeline = AggregateBuilder::new()
            .matching(("_id", &id))
            .lookup(MDB_COLL_NAME_USERS, "creator", "_id", "creator")
            .unwind("$creator", None, None)
            .build();
        let mut doc = collection.aggregate(pipeline, None).await?;
        match doc.next().await {
            Some(r) => Ok(Some(from_document(r?)?)),
            None => Ok(None),
        }
    }

    pub async fn list_customer(&self, start: i64, limit: i64) -> Result<Cursor<Document>> {
        let collection = self.database.collection(MDB_COLL_CUSTOMERS);
        let pipeline = AggregateBuilder::new()
            .skip(start)
            .limit(limit)
            .lookup(MDB_COLL_NAME_USERS, "creator", "_id", "creator")
            .unwind("$creator", None, None)
            .build();
        Ok(collection.aggregate(pipeline, None).await?)
    }

    pub async fn count_customers(&self) -> Result<usize> {
        let collection = self.database.collection(MDB_COLL_CUSTOMERS);
        Ok(collection.estimated_document_count(None).await? as usize)
    }

    pub async fn new_customer(
        &self,
        user_id: UserId,
        new: NewCustomer,
    ) -> Result<CustomerResponse> {
        let col = self.database.collection(MDB_COLL_CUSTOMERS);
        let new_customer = Customer::new(new, user_id.clone());
        let doc = to_document(&new_customer)?;

        let _ = col.insert_one(doc, None).await?;
        Ok(self
            .get_customer_by_id(new_customer.get_id().clone())
            .await?
            .unwrap())
    }

    pub async fn update_customer(
        &self,
        id: CustomerId,
        customer_update: CustomerUpdate,
    ) -> Result<bool> {
        let col = self.database.collection(MDB_COLL_CUSTOMERS);
        let filter = doc! { "_id": id };
        let mut update = bson::Document::new();
        update.insert("$set", bson::to_bson(&customer_update)?);

        let _ = col.update_one(filter, update, None).await?;
        Ok(true)
    }
}
