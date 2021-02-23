#[derive(Default)]
pub struct WorkAccountQuery;

#[Object]
impl WorkAccountQuery {
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
}

#[derive(Default)]
pub struct WorkAccountMutation;

#[Object]
impl WorkAccountMutation {
    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::WorkAccountModerator")
    )))]
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
}


