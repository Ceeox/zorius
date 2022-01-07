use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Result,
};
use futures::{stream, StreamExt};
use uuid::Uuid;

use crate::{
    api::{calc_list_params, claim::Claim, database},
    models::customer::{
        count_customers, customer_by_id, delete_customer, list_customers, new_customer,
        update_customer,
    },
    view::customer::{Customer, NewCustomer, UpdateCustomer},
};

#[derive(Default)]
pub struct CustomerQuery;

#[Object]
impl CustomerQuery {
    async fn get_customer_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Customer>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(&ctx)?.db();
        if let Some(customer) = customer_by_id(db, id).await? {
            return Ok(Some(customer.into()));
        }
        Ok(None)
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
        let db = database(ctx)?.db();
        let count = count_customers(db).await? as usize;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let (start, end, limit) = calc_list_params(count, after, before, first, last);

                let customers = list_customers(db, start, limit).await?;

                let mut connection = Connection::new(start > 0, end < count);
                connection
                    .append_stream(
                        stream::iter(customers)
                            .enumerate()
                            .map(|(n, customer)| Edge::new(n + start, customer.into())),
                    )
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
    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::WorkReportModerator")
    // )))]
    async fn new_customer(&self, ctx: &Context<'_>, new: NewCustomer) -> Result<Option<Customer>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(ctx)?.db();

        if let Some(customer) = new_customer(db, new).await? {
            return Ok(Some(customer.into()));
        }
        Ok(None)
    }

    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::WorkReportModerator")
    // )))]
    async fn update_customer(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        update: UpdateCustomer,
    ) -> Result<Option<Customer>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(ctx)?.db();

        if let Some(customer) = update_customer(db, id, update).await? {
            return Ok(Some(customer.into()));
        }
        Ok(None)
    }

    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::MerchandiseModerator")
    // )))]
    async fn delete_customer(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(ctx)?.db();

        Ok(delete_customer(db, id).await? >= 1)
    }
}
