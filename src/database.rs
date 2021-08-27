use std::path::Path;

use async_graphql::Result;
use log::{error, info};
use sqlx::migrate::Migrator;
use sqlx::{Pool, Postgres, Row};

use crate::view::users::{NewUser, UserUpdate};
use crate::{
    config::CONFIG,
    models::{
        roles::{Role, RoleUpdateMode},
        users::{User, UserId},
    },
};

pub struct Database {
    database: Pool<Postgres>,
}

impl Database {
    pub async fn new(database: Pool<Postgres>) -> Self {
        info!("Running migrations...");
        let m = Migrator::new(Path::new("./migrations"))
            .await
            .expect("Failed to run migrates");
        m.run(&database).await.expect("Failed to run migrates");

        let _self = Self { database };

        let admin_user = NewUser {
            email: CONFIG.admin_user.email.clone(),
            password: CONFIG.admin_user.password.clone(),
            firstname: CONFIG.admin_user.firstname.clone(),
            lastname: CONFIG.admin_user.lastname.clone(),
        };

        if User::get_dbuser_by_email(&_self.database, admin_user.email.clone())
            .await
            .is_ok()
        {
            return _self;
        }

        match User::new_user(&_self.database, admin_user).await {
            Ok(_) => {}
            Err(e) => error!("Failed to create admin user: {:?}", e),
        }

        _self
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.database
    }
    /*
        pub async fn update_role(
            &self,
            user_id: uuid::Uuid,
            role_update: RoleUpdateMode,
            role: Role,
        ) -> Result<()> {
            let _ = sqlx::query_file_as!(DBUser, "sql/update_role.sql", user_id, password_hash,)
                .fetch_one(&self.database)
                .await?;
            Ok(())
        }
    */

    /*
    pub async fn get_work_report_by_id(
        &self,
        id: WorkReportId,
        user_id: UserId,
    ) -> Result<WorkReport> {
        let collection = self.database.collection(MDB_COLL_WORK_REPORTS);
        let pipeline = AggregateBuilder::new()
            .matching(vec![("_id", id), ("user_id", user_id)])
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

    pub async fn get_todays_work_reports(
        &self,
        id: WorkReportId,
        user_id: UserId,
    ) -> Result<WorkReport> {
        let collection = self.database.collection(MDB_COLL_WORK_REPORTS);
        let pipeline = AggregateBuilder::new()
            .matching(vec![("_id", id), ("user_id", user_id)])
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
            .lookup(
                MDB_COLL_PROJECTS,
                "customer.project_ids",
                "_id",
                "customer.projects",
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
            .matching(vec![("user_id", user_id)])
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
            .lookup(
                MDB_COLL_PROJECTS,
                "customer.project_ids",
                "_id",
                "customer.projects",
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

    pub async fn new_work_report(&self, user_id: UserId, new: NewWorkReport) -> Result<WorkReport> {
        let col = self.database.collection(MDB_COLL_WORK_REPORTS);
        let new_work_report = DBWorkReport::new(user_id.clone(), new);
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

    pub async fn get_customer_by_id(&self, id: CustomerId) -> Result<Customer> {
        let collection = self.database.collection(MDB_COLL_CUSTOMERS);
        let pipeline = AggregateBuilder::new()
            .matching(vec![("_id", &id)])
            .lookup(MDB_COLL_NAME_USERS, "creator_id", "_id", "creator")
            .unwind("$creator", None, None)
            .lookup(MDB_COLL_PROJECTS, "project_ids", "_id", "projects")
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
            .lookup(MDB_COLL_PROJECTS, "project_ids", "_id", "projects")
            .build();
        Ok(collection.aggregate(pipeline, None).await?)
    }

    pub async fn count_customers(&self) -> Result<usize> {
        let collection = self.database.collection(MDB_COLL_CUSTOMERS);
        Ok(collection.estimated_document_count(None).await? as usize)
    }

    pub async fn new_customer(&self, user_id: UserId, new: NewCustomer) -> Result<Customer> {
        let col = self.database.collection(MDB_COLL_CUSTOMERS);
        let new_customer = DBCustomer::new(new, user_id.clone());
        let doc = to_document(&new_customer)?;

        let _ = col.insert_one(doc, None).await?;
        Ok(self
            .get_customer_by_id(new_customer.get_id().clone())
            .await?)
    }

    pub async fn update_customer(
        &self,
        id: CustomerId,
        customer_update: UpdateCustomer,
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

    pub async fn get_project_by_id(&self, id: ProjectId) -> Result<Project> {
        let collection = self.database.collection(MDB_COLL_PROJECTS);
        let pipeline = AggregateBuilder::new().matching(vec![("_id", &id)]).build();
        let mut doc = collection.aggregate(pipeline, None).await?;
        match doc.next().await {
            Some(r) => Ok(from_document(r?)?),
            None => Err(Error::new("project wasn't found")),
        }
    }

    pub async fn list_projects(&self, start: i64, limit: i64) -> Result<Cursor<Document>> {
        let collection = self.database.collection(MDB_COLL_PROJECTS);
        let pipeline = AggregateBuilder::new().skip(start).limit(limit).build();
        Ok(collection.aggregate(pipeline, None).await?)
    }

    pub async fn count_projects(&self) -> Result<usize> {
        let collection = self.database.collection(MDB_COLL_PROJECTS);
        Ok(collection.estimated_document_count(None).await? as usize)
    }

    pub async fn new_project(&self, new: NewProject) -> Result<Project> {
        let col = self.database.collection(MDB_COLL_PROJECTS);
        let new_project = DBProject::new(new);
        let doc = to_document(&new_project)?;

        let _ = col.insert_one(doc, None).await?;
        Ok(self.get_project_by_id(new_project.get_id().clone()).await?)
    }

    pub async fn update_project(
        &self,
        id: ProjectId,
        user_id: UserId,
        project_update: UpdateProject,
    ) -> Result<Project> {
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
    */
}
