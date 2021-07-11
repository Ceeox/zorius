use std::usize;

use async_graphql::{Error, Result};
use sqlx::{Pool, Postgres};

use crate::models::user::{DBUser, NewUser, User, UserId, UserUpdate};

pub struct Database {
    database: Pool<Postgres>,
}

impl Database {
    pub async fn new(database: Pool<Postgres>) -> Self {
        sqlx::migrate!("./migrations")
            .run(&database)
            .await
            .expect("failed to run migrations");
        Self { database }
    }

    pub async fn get_dbuser_by_id(&self, id: uuid::Uuid) -> Result<DBUser> {
        let user = sqlx::query_as(
            r#"
                SELECT *
                FROM users
                WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.database)
        .await?;
        Ok(user)
    }

    pub async fn get_dbuser_by_email(&self, email: String) -> Result<DBUser> {
        let user = sqlx::query_as(
            r#"
                SELECT *
                FROM users
                WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_one(&self.database)
        .await?;
        Ok(user)
    }

    pub async fn list_users(&self, start: i64, limit: i64) -> Result<Vec<DBUser>> {
        let user = sqlx::query_as(
            r#"
                SELECT *
                FROM users
                LIMIT $1
                OFFSET $1
            "#,
        )
        .bind(limit)
        .bind(start)
        .fetch_all(&self.database)
        .await?;
        Ok(user)
    }

    pub async fn count_users(&self) -> Result<usize> {
        todo!();
    }

    pub async fn new_user(&self, new_user: NewUser) -> Result<User> {
        let user = DBUser::new(new_user);
        // let rec = sqlx::query!(
        //     r#"
        //         INSERT INTO users (
        //             user as "user: DBUser"
        //         )
        //         VALUES ( $1 )
        //         RETURNING *
        //     "#,
        //     user
        // )
        // .fetch_one(&self.database)
        // .await?;
        Ok(user.into())
    }

    pub async fn update_user(&self, id: UserId, user_update: UserUpdate) -> Result<User> {
        todo!()
    }

    pub async fn reset_password(&self, user_id: uuid::Uuid, password_hash: &str) -> Result<()> {
        let _ = sqlx::query_as::<_, DBUser>("UPDATE users SET password_hash = $1 WHERE id = $1")
            .bind(password_hash)
            .bind(user_id)
            .fetch_one(&self.database)
            .await;
        Ok(())
    }

    /*
    pub async fn get_intern_merch_by_id(
        &self,
        id: InternMerchandiseId,
    ) -> Result<InternMerchandise> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        let pipeline = AggregateBuilder::new()
            .matching(vec![("_id", id)])
            .lookup(MDB_COLL_NAME_USERS, "orderer_id", "_id", "orderer")
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
    ) -> Result<InternMerchandise> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        let pipeline = AggregateBuilder::new()
            .matching(vec![("merchandise_id", merchandise_id)])
            .lookup(MDB_COLL_NAME_USERS, "orderer_id", "_id", "orderer")
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
            .sort(vec![("created_date", SortOrder::DESC)])
            .lookup(MDB_COLL_NAME_USERS, "orderer_id", "_id", "orderer")
            .lookup(
                MDB_COLL_NAME_USERS,
                "project_leader_id",
                "_id",
                "project_leader",
            )
            .unwind("$orderer", None, None)
            .unwind("$project_leader", None, None)
            .build();
        Ok(collection.aggregate(pipeline, None).await?)
    }

    pub async fn count_intern_merch(&self) -> Result<usize> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        Ok(collection.estimated_document_count(None).await? as usize)
    }

    pub async fn new_intern_merch(
        &self,
        new_intern_merch: DBInternMerchandise,
    ) -> Result<InternMerchandise> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        let id = new_intern_merch.id.clone();
        let doc = to_document(&new_intern_merch)?;
        let _ = collection.insert_one(doc, None).await?;
        Ok(self.get_intern_merch_by_id(id).await?)
    }

    pub async fn update_intern_merch(
        &self,
        id: InternMerchandiseId,
        update: InternMerchandiseUpdate,
    ) -> Result<InternMerchandise> {
        let collection = self.database.collection(MDB_COLL_INTERN_MERCH);
        let filter = doc! {"_id": id.clone()};
        let update = doc! {"$set": bson::to_bson(&update)?};
        let _ = collection.update_one(filter, update, None).await?;
        Ok(self.get_intern_merch_by_id(id).await?)
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
