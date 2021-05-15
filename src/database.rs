use async_graphql::{Error, Result};
use bson::{doc, from_document, oid::ObjectId, to_document, Bson, Document};
use futures::StreamExt;
use mongodb::{
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Client, Cursor, Database as MongoDB,
};

use crate::{
    helper::AggregateBuilder,
    models::{
        customer::{Customer, CustomerId, CustomerResponse, CustomerUpdate, NewCustomer},
        intern_merchandise::{
            InternMerchResponse, InternMerchandise, InternMerchandiseId, InternMerchandiseUpdate,
        },
        project::{NewProject, Project, ProjectId, ProjectResponse, ProjectUpdate},
        user::{NewUser, User, UserId, UserUpdate},
        work_report::{
            NewWorkReport, WorkReport, WorkReportId, WorkReportResponse, WorkReportUpdate,
        },
    },
};

pub(crate) static MDB_COLL_NAME_USERS: &str = "users";
pub(crate) static MDB_COLL_WORK_REPORTS: &str = "work_reports";
pub(crate) static MDB_COLL_INTERN_MERCH: &str = "merchandise_intern";
pub(crate) static MDB_COLL_CUSTOMERS: &str = "customers";
pub(crate) static MDB_COLL_PROJECTS: &str = "projects";

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

    pub async fn get_user_by_id(&self, id: UserId) -> Result<User> {
        let col = self.database.collection(MDB_COLL_NAME_USERS);
        let filter = doc! {"_id": id};
        match col.find_one(filter, None).await? {
            None => Err(Error::new("user wasn't found")),
            Some(doc) => Ok(from_document::<User>(doc)?),
        }
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<User> {
        let col = self.database.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "email": email.clone() };
        match col.find_one(filter, None).await? {
            None => Err(Error::new("user wasn't found")),
            Some(doc) => Ok(from_document::<User>(doc)?),
        }
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
    ) -> Result<InternMerchResponse> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        let pipeline = AggregateBuilder::new()
            .matching(("_id", id))
            .lookup(MDB_COLL_NAME_USERS, "orderer", "_id", "orderer")
            .lookup(
                MDB_COLL_NAME_USERS,
                "project_leader_id",
                "_id",
                "project_leader",
            )
            .unwind("$orderer", None, None)
            .unwind("$project_leader", None, None)
            .build();
        let mut doc = collection.aggregate(pipeline, None).await?;
        match doc.next().await {
            Some(r) => Ok(from_document(r?)?),
            None => Err(Error::new("intern merch wasn't found")),
        }
    }

    pub async fn get_intern_merch_by_merch_id(
        &self,
        merchandise_id: i32,
    ) -> Result<InternMerchResponse> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        let pipeline = AggregateBuilder::new()
            .matching(("merchandise_id", merchandise_id))
            .lookup(MDB_COLL_NAME_USERS, "orderer", "_id", "orderer")
            .lookup(
                MDB_COLL_NAME_USERS,
                "project_leader_id",
                "_id",
                "project_leader",
            )
            .unwind("$orderer", None, None)
            .unwind("$project_leader", None, None)
            .build();
        let mut doc = collection.aggregate(pipeline, None).await?;
        match doc.next().await {
            Some(r) => Ok(from_document(r?)?),
            None => Err(Error::new("intern merch wasn't found")),
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
    ) -> Result<InternMerchResponse> {
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
            None => Err(Error::new("intern merch wasn't found")),
            Some(doc) => Ok(from_document(doc)?),
        }
    }

    pub async fn delete_intern_merch(&self, id: InternMerchandiseId) -> Result<()> {
        let col = self.database.collection(MDB_COLL_INTERN_MERCH);
        let query = doc! {"_id": id};
        let _ = col.delete_one(query, None).await?;
        Ok(())
    }

    pub async fn get_work_report_by_id(
        &self,
        id: WorkReportId,
        user_id: UserId,
    ) -> Result<WorkReportResponse> {
        let collection = self.database.collection(MDB_COLL_WORK_REPORTS);
        let pipeline = AggregateBuilder::new()
            .matching(("_id", id))
            .matching(("user_id", user_id))
            .lookup(MDB_COLL_NAME_USERS, "user_id", "_id", "user")
            .lookup(MDB_COLL_PROJECTS, "project_id", "_id", "project")
            .unwind("$project", None, None)
            .lookup(
                MDB_COLL_NAME_USERS,
                "project.creator_id",
                "_id",
                "project.creator",
            )
            .lookup(MDB_COLL_CUSTOMERS, "customer_id", "_id", "customer")
            .unwind("$customer", None, None)
            .lookup(
                MDB_COLL_NAME_USERS,
                "customer.creator_id",
                "_id",
                "customer.creator",
            )
            .unwind("$user", None, None)
            .unwind("$project.creator", None, None)
            .unwind("$customer.creator", None, None)
            .build();
        let mut doc = collection.aggregate(pipeline, None).await?;

        match doc.next().await {
            Some(r) => Ok(from_document(r?)?),
            None => Err(Error::new("work report wasn't found")),
        }
    }

    pub async fn has_ref_to_work_report(&self, key: &str, value: ObjectId) -> Result<bool> {
        let collection = self.database.collection(MDB_COLL_WORK_REPORTS);
        let filter = doc! { key: value };
        match collection.find(filter, None).await?.next().await {
            Some(_) => Ok(true),
            None => Ok(false),
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
            .lookup(MDB_COLL_PROJECTS, "project_id", "_id", "project")
            .unwind("$project", None, None)
            .lookup(
                MDB_COLL_NAME_USERS,
                "project.creator_id",
                "_id",
                "project.creator",
            )
            .lookup(MDB_COLL_CUSTOMERS, "customer_id", "_id", "customer")
            .unwind("$customer", None, None)
            .lookup(
                MDB_COLL_NAME_USERS,
                "customer.creator_id",
                "_id",
                "customer.creator",
            )
            .unwind("$user", None, None)
            .unwind("$project.creator", None, None)
            .unwind("$customer.creator", None, None)
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

        let res = col.insert_one(doc, None).await?;
        if res
            .inserted_id
            .as_object_id()
            .ne(&Some(new_work_report.get_id()))
        {
            return Err(Error::new("Couldn't save the work_report into database"));
        }

        Ok(self
            .get_work_report_by_id(new_work_report.get_id().clone(), user_id)
            .await?)
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

    pub async fn delete_work_report(&self, id: WorkReportId) -> Result<()> {
        let col = self.database.collection(MDB_COLL_WORK_REPORTS);
        let query = doc! {"_id": id};
        let _ = col.delete_one(query, None).await?;
        Ok(())
    }

    pub async fn get_customer_by_id(&self, id: CustomerId) -> Result<CustomerResponse> {
        let collection = self.database.collection(MDB_COLL_CUSTOMERS);
        let pipeline = AggregateBuilder::new()
            .matching(("_id", &id))
            .lookup(MDB_COLL_NAME_USERS, "creator_id", "_id", "creator")
            .unwind("$creator", None, None)
            .build();
        let mut doc = collection.aggregate(pipeline, None).await?;
        match doc.next().await {
            Some(r) => Ok(from_document(r?)?),
            None => Err(Error::new("customer wasn't found")),
        }
    }

    pub async fn list_customer(&self, start: i64, limit: i64) -> Result<Cursor<Document>> {
        let collection = self.database.collection(MDB_COLL_CUSTOMERS);
        let pipeline = AggregateBuilder::new()
            .skip(start)
            .limit(limit)
            .lookup(MDB_COLL_NAME_USERS, "creator_id", "_id", "creator")
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
            .await?)
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

    pub async fn delete_customer(&self, id: CustomerId) -> Result<()> {
        let col = self.database.collection(MDB_COLL_CUSTOMERS);
        let query = doc! {"_id": id};
        let _ = col.delete_one(query, None).await?;
        Ok(())
    }

    pub async fn get_project_by_id(&self, id: ProjectId) -> Result<ProjectResponse> {
        let collection = self.database.collection(MDB_COLL_PROJECTS);
        let pipeline = AggregateBuilder::new()
            .matching(("_id", &id))
            .lookup(MDB_COLL_NAME_USERS, "creator_id", "_id", "creator")
            .unwind("$creator", None, None)
            .build();
        let mut doc = collection.aggregate(pipeline, None).await?;
        match doc.next().await {
            Some(r) => Ok(from_document(r?)?),
            None => Err(Error::new("project wasn't found")),
        }
    }

    pub async fn list_projects(&self, start: i64, limit: i64) -> Result<Cursor<Document>> {
        let collection = self.database.collection(MDB_COLL_PROJECTS);
        let pipeline = AggregateBuilder::new()
            .skip(start)
            .limit(limit)
            .lookup(MDB_COLL_NAME_USERS, "creator_id", "_id", "creator")
            .unwind("$creator", None, None)
            .build();
        Ok(collection.aggregate(pipeline, None).await?)
    }

    pub async fn count_projects(&self) -> Result<usize> {
        let collection = self.database.collection(MDB_COLL_PROJECTS);
        Ok(collection.estimated_document_count(None).await? as usize)
    }

    pub async fn new_project(&self, user_id: UserId, new: NewProject) -> Result<ProjectResponse> {
        let col = self.database.collection(MDB_COLL_PROJECTS);
        let new_project = Project::new(user_id.clone(), new);
        let doc = to_document(&new_project)?;

        let _ = col.insert_one(doc, None).await?;
        Ok(self.get_project_by_id(new_project.get_id().clone()).await?)
    }

    pub async fn update_project(
        &self,
        id: ProjectId,
        user_id: UserId,
        project_update: ProjectUpdate,
    ) -> Result<ProjectResponse> {
        let col = self.database.collection(MDB_COLL_PROJECTS);
        let filter = doc! { "_id": id.clone(), "creator_id": user_id };
        let mut update = bson::Document::new();
        update.insert("$set", bson::to_bson(&project_update)?);

        let _ = col.update_one(filter, update, None).await?;

        Ok(self.get_project_by_id(id).await?)
    }

    pub async fn delete_project(&self, id: ProjectId) -> Result<()> {
        let col = self.database.collection(MDB_COLL_PROJECTS);
        let query = doc! {"_id": id};
        let _ = col.delete_one(query, None).await?;
        Ok(())
    }
}
