use async_graphql::{
    guard::Guard,
    validators::{Email, StringMaxLength, StringMinLength},
    Context, Error, Object, Result,
};
use bson::{doc, from_document, oid::ObjectId};
use chrono::{Duration, NaiveDate, Utc};
use futures::{future, StreamExt, TryStreamExt};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use mongodb::{options::FindOptions, Collection, Cursor};

use crate::{
    config::CONFIG,
    models::{
        auth::LoginResult,
        merchandise::intern_merchandise::MerchandiseIntern,
        roles::{Role, RoleGuard, Roles},
        user::{Claim, User, UserId},
        work_record::{workday::Workday, WorkAccount},
        work_report::{
            customer::{Customer, CustomerId},
            project::{Project, ProjectId},
            WorkReport, WorkReportId,
        },
    },
    API_VERSION,
};

use super::{
    database, is_autherized, MDB_COLL_INTERN_MERCH, MDB_COLL_NAME_USERS, MDB_COLL_ROLES,
    MDB_COLL_WORK_ACCOUNTS, MDB_COLL_WORK_REPORTS,
};
pub struct RootQuery;

#[Object]
impl RootQuery {
    async fn api_version(&self) -> &str {
        API_VERSION
    }

    pub async fn login(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(Email))] email: String,
        #[graphql(validator(and(StringMinLength(length = "0"), StringMaxLength(length = "64"))))]
        password: String,
    ) -> Result<LoginResult> {
        let err = Error::new("email or password wrong!");
        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "email": email.clone() };
        let user: User = match collection.find_one(filter, None).await? {
            None => {
                return Err(err);
            }
            Some(r) => from_document(r).unwrap(),
        };

        if !user.is_password_correct(&password) {
            return Err(err);
        }
        let claims = Claim {
            sub: email,
            user_id: user.get_id().clone(),
            exp: (Utc::now() + Duration::days(30)).timestamp() as usize,
        };
        let key = &EncodingKey::from_secret(&CONFIG.secret_key.as_bytes());
        let token = jsonwebtoken::encode(&Header::new(Algorithm::HS512), &claims, key)?;

        Ok(LoginResult {
            token: token,
            expires_at: claims.exp,
            user_id: user.get_id().to_owned(),
        })
    }

    async fn table_data(&self, ctx: &Context<'_>) -> Result<Vec<MerchandiseIntern>> {
        is_autherized(ctx)?;
        let collection: Collection = database(ctx)?.collection("merchandise_intern");
        let find_opt = Some(FindOptions::builder().limit(50).build());
        let cursor = collection.find(None, find_opt).await?;
        let res = cursor
            .filter_map(|doc| async move {
                match doc {
                    Err(_) => None,
                    Ok(r) => Some(from_document::<MerchandiseIntern>(r)),
                }
            })
            .try_collect::<Vec<_>>()
            .await?;

        Ok(res)
    }

    async fn get_order(&self, ctx: &Context<'_>, id: ObjectId) -> Result<MerchandiseIntern> {
        is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_INTERN_MERCH);
        let filter = doc! { "_id": id };
        match collection.find_one(Some(filter), None).await? {
            None => return Err(Error::new("intern order not found")),
            Some(r) => Ok(from_document(r)?),
        }
    }

    async fn get_user(&self, ctx: &Context<'_>, user_id: UserId) -> Result<User> {
        is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "_id": user_id };
        match collection.find_one(filter, None).await? {
            None => return Err(Error::new("specified user not found")),
            Some(r) => Ok(from_document(r)?),
        }
    }

    async fn get_users(&self, ctx: &Context<'_>, user_ids: Vec<UserId>) -> Result<Vec<User>> {
        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "_id": {
                "$in": bson::to_bson(&user_ids)?,
            }
        };
        let cursor: Cursor = collection.find(filter, None).await?;
        let res = cursor
            .filter(|doc| future::ready(doc.is_ok()))
            .map(|doc| from_document::<User>(doc.unwrap()).unwrap())
            .filter(|user| future::ready(!user.is_deleted()))
            .collect::<Vec<_>>()
            .await;

        Ok(res)
    }

    async fn list_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let find_opt = Some(FindOptions::builder().limit(50).build());
        let cursor: Cursor = collection.find(None, find_opt).await?;
        let res = cursor
            .filter_map(|doc| async move {
                match doc {
                    Err(_) => None,
                    Ok(r) => Some(from_document::<User>(r)),
                }
            })
            .try_collect::<Vec<_>>()
            .await?;

        return Ok(res);
    }

    async fn get_workday(&self, ctx: &Context<'_>, date: NaiveDate) -> Result<Option<Workday>> {
        let user_id = is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_WORK_ACCOUNTS);
        let pl = vec![
            doc! {"$unwind": "$workdays"},
            doc! {"$match": {"user_id": user_id.clone(), "workdays.date":  date.to_string()}},
            doc! {"$replaceRoot": {"newRoot": "$workdays"}},
        ];
        let mut wd = collection.aggregate(pl, None).await?;
        match wd.next().await {
            Some(r) => Ok(Some(from_document(r?)?)),
            None => Ok(None),
        }
    }

    async fn get_workdays(
        &self,
        ctx: &Context<'_>,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<Workday>> {
        let user_id = is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_WORK_ACCOUNTS);
        let pl = vec![
            doc! {"$unwind": "$workdays"},
            doc! {"$match": {
                "user_id": user_id.clone(),
                    "workdays.date": {
                        "$lte": end_date.to_string(),
                        "$gte": start_date.to_string(),
                    }
                }
            },
            doc! {"$replaceRoot": {"newRoot": "$workdays"}},
        ];
        Ok(collection
            .aggregate(pl, None)
            .await?
            .filter_map(|item| async move {
                if item.is_ok() {
                    match from_document(item.unwrap()) {
                        Ok(r) => Some(r),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<Workday>>()
            .await)
    }

    async fn get_workaccount(&self, ctx: &Context<'_>) -> Result<Option<WorkAccount>> {
        let user_id = is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_WORK_ACCOUNTS);
        let filter = doc! { "user_id": user_id.clone() };
        let wd = collection.find_one(filter, None).await?;
        match wd {
            Some(r) => Ok(Some(from_document(r)?)),
            None => Ok(None),
        }
    }

    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::WorkAccountModerator")
    )))]
    async fn get_workaccounts(
        &self,
        ctx: &Context<'_>,
        user_ids: Vec<UserId>,
    ) -> Result<Option<WorkAccount>> {
        let _ = is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_WORK_ACCOUNTS);
        let filter = doc! { "user_id": {
                "$in": bson::to_bson(&user_ids)?,
            }
        };
        let wd = collection.find_one(filter, None).await?;
        match wd {
            Some(r) => Ok(Some(from_document(r)?)),
            None => Ok(None),
        }
    }

    async fn get_workreport(
        &self,
        ctx: &Context<'_>,
        work_report_id: WorkReportId,
    ) -> Result<Option<WorkReport>> {
        let user_id = is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_WORK_REPORTS);
        let filter = doc! { "user_id": user_id.clone(), "id": work_report_id };
        match collection.find_one(filter, None).await? {
            Some(r) => Ok(Some(from_document(r)?)),
            None => Err(Error::new("work_report_id not found")),
        }
    }

    async fn get_project(
        &self,
        ctx: &Context<'_>,
        project_id: ProjectId,
    ) -> Result<Option<Project>> {
        let _ = is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_WORK_REPORTS);
        let filter = doc! {
            "_id": project_id
        };
        match collection.find_one(filter, None).await? {
            Some(r) => Ok(Some(from_document(r)?)),
            None => Err(Error::new("customer not found")),
        }
    }

    async fn get_customer(
        &self,
        ctx: &Context<'_>,
        customer_id: CustomerId,
    ) -> Result<Option<Customer>> {
        let _ = is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_WORK_REPORTS);
        let filter = doc! {
            "_id": customer_id
        };
        match collection.find_one(filter, None).await? {
            Some(r) => Ok(Some(from_document(r)?)),
            None => Err(Error::new("customer not found")),
        }
    }

    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::RoleModerator")
    )))]
    async fn list_roles(&self, ctx: &Context<'_>, user_id: UserId) -> Result<Option<Roles>> {
        let _ = is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_ROLES);
        let filter = doc! {
            "user_id": user_id
        };
        match collection.find_one(filter, None).await? {
            Some(r) => Ok(Some(from_document(r)?)),
            None => Err(Error::new("user in roles not found")),
        }
    }
}
