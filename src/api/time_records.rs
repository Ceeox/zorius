#[derive(Default)]
pub struct TimeRecordQuery;

#[Object]
impl TimeRecordQuery {
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
}


#[derive(Default)]
pub struct TimeRecordMutation;

#[Object]
impl TimeRecordMutation {
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
}
