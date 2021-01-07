use async_graphql::{validators::StringMaxLength, Context, Error, Object, Result};
use bson::{doc, from_document, to_document};

use crate::{
    helper::validators::Password,
    models::user::{NewUser, User, UserId},
    models::{
        merchandise::intern_merchandise::{MerchandiseIntern, NewMerchandiseIntern},
        work_record::{workday::Workday, WorkAccount},
    },
};

use super::{
    database, is_autherized, MDB_COLL_INTERN_MERCH, MDB_COLL_NAME_USERS, MDB_COLL_WORK_ACCOUNTS,
};

pub struct RootMutation;

#[Object]
impl RootMutation {
    /*
        async fn new_intern_order(
            ctx: &Context,
            new_intern_order: NewInternMerchandiseQuery,
        ) -> FieldResult<InternMerchandiseResponse> {
            InternMerchandiseMutation::new_intern_order(ctx, new_intern_order).await
        }

        async fn update_intern_order(
            ctx: &Context,
            order_id: ObjectId,
            inter_update: UpdateInternMerchandiseQuery,
        ) -> FieldResult<InternMerchandiseResponse> {
            InternMerchandiseMutation::update_intern_order(ctx, order_id, inter_update).await
        }
    */
    async fn register(&self, ctx: &Context<'_>, new_user: NewUser) -> Result<User> {
        let user = User::new(new_user);
        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let doc = to_document(&user)?;
        let _ = collection.insert_one(doc.clone(), None).await?;
        Ok(user.into())
    }

    async fn reset_password(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(StringMaxLength(length = "64")))] old_password: String,
        #[graphql(validator(Password))] new_password: String,
    ) -> Result<bool> {
        let user_id = is_autherized(ctx)?;

        let collection = database(ctx)?.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "_id": user_id };
        let mut user: User = match collection.find_one(filter.clone(), None).await? {
            None => return Err(Error::new("specified user not found".to_owned())),
            Some(r) => from_document(r)?,
        };

        if !user.is_password_correct(&old_password) {
            return Err(Error::new("old password is wrong!".to_owned()));
        } else {
            user.change_password(&new_password);
        }

        let update = to_document(&user)?;
        let _ = collection.update_one(filter, update, None).await?;

        Ok(true)
    }

    async fn new_work_account(
        &self,
        ctx: &Context<'_>,
        user_id: UserId,
        default_work_target: Option<i64>,
    ) -> Result<WorkAccount> {
        let _ = is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_WORK_ACCOUNTS);

        let filter = doc! { "user_id": user_id.clone() };
        match collection.find_one(filter, None).await? {
            Some(_) => return Err(Error::new("work account for the user id already exists!")),
            None => {}
        }

        let new_workaccount = WorkAccount::new(user_id, default_work_target);
        let wa_id = new_workaccount.get_id().clone();
        let insert = to_document(&new_workaccount)?;
        let _ = collection.insert_one(insert, None).await?;

        let filter = doc! { "_id": wa_id };
        let wa = collection.find_one(filter, None).await?.unwrap();
        Ok(from_document(wa)?)
    }

    async fn workday_start(&self, ctx: &Context<'_>) -> Result<Workday> {
        let user_id = is_autherized(ctx)?;

        let collection = database(ctx)?.collection(MDB_COLL_WORK_ACCOUNTS);
        let filter = doc! { "user_id": user_id };
        let wa_doc = collection.find_one(filter.clone(), None).await?.unwrap();
        let mut wa: WorkAccount = from_document(wa_doc)?;

        wa.start_workday();

        let update = to_document(&wa)?;
        let _ = collection.update_one(filter, update, None).await?;

        let wd = wa.get_today_workday().unwrap();
        Ok(wd)
    }

    async fn workday_pause(&self, ctx: &Context<'_>) -> Result<Workday> {
        let user_id = is_autherized(ctx)?;

        let collection = database(ctx)?.collection(MDB_COLL_WORK_ACCOUNTS);
        let filter = doc! { "user_id": user_id };
        let wa_doc = collection.find_one(filter.clone(), None).await?.unwrap();
        let mut wa: WorkAccount = from_document(wa_doc)?;

        wa.pause();

        let update = to_document(&wa)?;
        let _ = collection.update_one(filter, update, None).await?;

        let wd = wa.get_today_workday().unwrap();
        Ok(wd)
    }

    async fn workday_resume(&self, ctx: &Context<'_>) -> Result<Workday> {
        let user_id = is_autherized(ctx)?;

        let collection = database(ctx)?.collection(MDB_COLL_WORK_ACCOUNTS);
        let filter = doc! { "user_id": user_id };
        let wa_doc = collection.find_one(filter.clone(), None).await?.unwrap();
        let mut wa: WorkAccount = from_document(wa_doc)?;

        wa.resume_work();

        let update = to_document(&wa)?;
        let _ = collection.update_one(filter, update, None).await?;

        let wd = wa.get_today_workday().unwrap();

        Ok(wd)
    }
    /*
    async fn update_user(&self, ctx: &Context<'_>, user_id: UserId, user_update: UpdateUser) -> Result<User> {
        let _ = is_autherized(ctx)?;
        let collection = ctx.db.collection(MDB_COLL_NAME_USERS);
        let filter = doc! { "_id": user_id };

        let mut replacement = to_document(&user_update)?.remove_null_keys();
        replacement.push(doc! { "$set": {"last_updated": Bson::DateTime(Utc::now())}});

        let options = FindOneAndUpdateOptions::builder()
            .return_document(Some(ReturnDocument::After))
            .build();

        let user: User = match collection
            .find_one_and_update(filter, replacement, Some(options))
            .await?
        {
            None => {
                return Err(Error::new("specified user not found")
            }
            Some(r) => from_document(r)?,
        };
        Ok(user.into())
    }
    */

    async fn new_merchandise_intern(
        &self,
        ctx: &Context<'_>,
        new_intern_merch: NewMerchandiseIntern,
    ) -> Result<MerchandiseIntern> {
        let _ = is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_INTERN_MERCH);

        let new_merch_intern = MerchandiseIntern::new(new_intern_merch);
        let im_id = new_merch_intern.get_id().clone();
        let insert = to_document(&new_merch_intern)?;
        let _ = collection.insert_one(insert, None).await?;

        let filter = doc! { "_id": im_id };
        let wa = collection.find_one(filter, None).await?.unwrap();
        Ok(from_document(wa)?)
    }
}
