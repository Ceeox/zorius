use async_graphql::{guard::Guard, Context, Error, Object, Result};
use bson::{doc, from_document, to_document};

use crate::{
    api::{claim::Claim, database},
    database::MDB_COLL_WORK_REPORTS,
    models::{
        roles::{Role, RoleGuard},
        work_report::customer::{Customer, CustomerAdd, CustomerId},
    },
};

#[derive(Default)]
pub struct CustomerQuery;

#[Object]
impl CustomerQuery {
    async fn get_customer(
        &self,
        ctx: &Context<'_>,
        customer_id: CustomerId,
    ) -> Result<Option<Customer>> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_WORK_REPORTS);
        let filter = doc! {
            "_id": customer_id
        };
        match collection.find_one(filter, None).await? {
            Some(r) => Ok(Some(from_document(r)?)),
            None => Err(Error::new("customer not found")),
        }
    }

    async fn list_customers(
        &self,
        ctx: &Context<'_>,
        customer_id: CustomerId,
    ) -> Result<Option<Customer>> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_WORK_REPORTS);
        let filter = doc! {
            "_id": customer_id
        };
        match collection.find_one(filter, None).await? {
            Some(r) => Ok(Some(from_document(r)?)),
            None => Err(Error::new("customer not found")),
        }
    }
}

#[derive(Default)]
pub struct CustomerMutation;

#[Object]
impl CustomerMutation {
    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::WorkReportModerator")
    )))]
    async fn new_customer(
        &self,
        ctx: &Context<'_>,
        name: String,
        identifier: String,
        note: Option<String>,
    ) -> Result<Customer> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();

        let collection = database(ctx)?.collection(MDB_COLL_WORK_REPORTS);
        let customer = Customer::new(CustomerAdd {
            name,
            identifier,
            note,
            creator: user_id.clone(),
            projects: None,
        });
        let insert = to_document(&customer)?;
        let _ = collection.insert_one(insert, None).await?;
        Ok(customer)
    }
}
