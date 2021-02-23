#[derive(Default)]
pub struct WorkReportQuery;

#[Object]
impl WorkReportQuery {
    async fn get_workreport(
        &self,
        ctx: &Context<'_>,
        work_report_id: WorkReportId,
    ) -> Result<Option<WorkReport>> {
        let user_id = is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_WORK_REPORTS);
        let filter = doc! { "user_id": user_id.clone(), "_id": work_report_id };
        match collection.find_one(filter, None).await? {
            Some(r) => Ok(Some(from_document(r)?)),
            None => Err(Error::new("work_report_id not found")),
        }
    }
}

#[derive(Default)]
pub struct WorkReportMutation;

#[Object]
impl WorkReportMutation {
    async fn new_workreport(
        &self,
        ctx: &Context<'_>,
        new_workreport: NewWorkReport,
    ) -> Result<WorkReport> {
        let user_id = is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_WORK_REPORTS);
        let wr = WorkReport::new(user_id, new_workreport);
        let insert = to_document(&wr)?;
        let _ = collection.insert_one(insert, None).await?;
        Ok(wr)
    }

    #[graphql(guard(RoleGuard(role = "Role::Admin")))]
    async fn update_workreport(
        &self,
        ctx: &Context<'_>,
        workreport_id: WorkReportId,
        workreport_update: WorkReportUpdate,
    ) -> Result<WorkReport> {
        let user_id = is_autherized(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_WORK_REPORTS);
        let filter = doc! { "_id": workreport_id, "user_id": user_id };

        let update = WorkReport::update(workreport_update)?;

        println!("{:#?}", update);

        let options = FindOneAndUpdateOptions::builder()
            .return_document(Some(ReturnDocument::After))
            .build();

        let wr = match collection
            .find_one_and_update(filter, update, Some(options))
            .await?
        {
            None => return Err(Error::new("specified workreport not found")),
            Some(r) => r,
        };
        Ok(from_document(wr)?)
    }
}