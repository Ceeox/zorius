use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    guard::Guard,
    Context, Error, Object, Result,
};
use bson::from_document;
use futures::StreamExt;

use crate::{
    api::{claim::Claim, database2},
    models::{
        customer::{Customer, CustomerId, NewCustomer, UpdateCustomer},
        roles::{Role, RoleGuard},
    },
};

#[derive(Default)]
pub struct CustomerQuery;

#[Object]
impl CustomerQuery {
    async fn get_customer_by_id(&self, ctx: &Context<'_>, id: CustomerId) -> Result<Customer> {
        let _ = Claim::from_ctx(ctx)?;
        Ok(database2(ctx)?.get_customer_by_id(id).await?)
    }

    async fn list_customers(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, Customer, EmptyFields, EmptyFields>> {
        let _ = Claim::from_ctx(ctx)?;
        let doc_count = database2(ctx)?.count_customers().await?;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = after.unwrap_or(0);
                let mut end = before.unwrap_or(doc_count);

                if let Some(first) = first {
                    end = (start + first).min(end);
                }
                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last };
                }
                let limit = (end - start) as i64;

                let cursor = database2(ctx)?.list_customer(start as i64, limit).await?;

                let mut connection = Connection::new(start > 0, end < doc_count);
                connection
                    .append_stream(cursor.enumerate().map(|(n, doc)| {
                        let customer = from_document::<Customer>(doc.unwrap()).unwrap();
                        Edge::with_additional_fields(n + start, customer, EmptyFields)
                    }))
                    .await;
                Ok(connection)
            },
        )
        .await
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
    async fn new_customer(&self, ctx: &Context<'_>, new: NewCustomer) -> Result<Customer> {
        let claim = Claim::from_ctx(ctx)?;
        let user_id = claim.user_id();

        Ok(database2(ctx)?
            .new_customer(user_id.to_owned(), new)
            .await?)
    }

    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::WorkReportModerator")
    )))]
    async fn update_customer(
        &self,
        ctx: &Context<'_>,
        id: CustomerId,
        update: UpdateCustomer,
    ) -> Result<Customer> {
        let _ = Claim::from_ctx(ctx)?;
        let _ = database2(ctx)?.update_customer(id.clone(), update).await?;

        Ok(database2(ctx)?.get_customer_by_id(id).await?)
    }

    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::MerchandiseModerator")
    )))]
    async fn delete_customer(&self, ctx: &Context<'_>, id: CustomerId) -> Result<bool> {
        let _ = Claim::from_ctx(ctx)?;
        if database2(ctx)?
            .has_ref_to_work_report("customer_id", id.clone())
            .await?
        {
            return Err(Error::new(
                "Can not delete Project with still a reference to a WorkReport",
            ));
        }
        let _ = database2(ctx)?.delete_customer(id).await?;

        Ok(true)
    }
}
