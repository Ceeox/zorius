use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Result,
};
use futures::{stream, StreamExt};

use crate::{
    api::{calc_list_params, claim::Claim, database},
    models::{
        customer::{self, CustomerEntity, CustomerId},
        project::ProjectEntity,
    },
    view::customer::{Customer, NewCustomer, UpdateCustomer},
};

#[derive(Default)]
pub struct CustomerQuery;

#[Object]
impl CustomerQuery {
    async fn get_customer_by_id(&self, ctx: &Context<'_>, id: CustomerId) -> Result<Customer> {
        let _ = Claim::from_ctx(ctx)?;
        let pool = database(&ctx)?.get_pool();
        let customer = CustomerEntity::get_customer_by_id(pool, id).await?;
        let mut res: Customer = customer.into();

        Ok(res)
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
        let pool = database(ctx)?.get_pool();
        let count = CustomerEntity::count_customers(pool).await? as usize;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let (start, end, limit) = calc_list_params(count, after, before, first, last);

                let customers =
                    CustomerEntity::list_customer(pool, start as i64, limit as i64).await?;
                let customers: Vec<Customer> = stream::iter(customers)
                    .filter_map(|db_customer| async move {
                        let mut projects =
                            match ProjectEntity::get_projects_for_customer_id(pool, db_customer.id)
                                .await
                            {
                                Ok(r) => r.into_iter().map(|project| project.into()).collect(),
                                Err(_) => return None,
                            };
                        let mut customer: Customer = db_customer.into();
                        std::mem::swap(&mut customer.projects, &mut projects);
                        Some(customer)
                    })
                    .collect()
                    .await;

                let mut connection = Connection::new(start > 0, end < count);
                connection
                    .append_stream(
                        stream::iter(customers)
                            .enumerate()
                            .map(|(n, customer)| Edge::new(n + start, customer)),
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
    async fn new_customer(&self, ctx: &Context<'_>, new_customer: NewCustomer) -> Result<Customer> {
        let _ = Claim::from_ctx(ctx)?;
        let pool = database(ctx)?.get_pool();

        Ok(CustomerEntity::new(pool, new_customer).await?.into())
    }

    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::WorkReportModerator")
    // )))]
    async fn update_customer(
        &self,
        ctx: &Context<'_>,
        id: CustomerId,
        update: UpdateCustomer,
    ) -> Result<Customer> {
        let _ = Claim::from_ctx(ctx)?;
        let pool = database(ctx)?.get_pool();

        Ok(CustomerEntity::update_customer(pool, id, update).await?)
    }

    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::MerchandiseModerator")
    // )))]
    async fn delete_customer(&self, ctx: &Context<'_>, id: CustomerId) -> Result<Customer> {
        let _ = Claim::from_ctx(ctx)?;
        let pool = database(ctx)?.get_pool();

        Ok(CustomerEntity::delete_customer(pool, id).await?.into())
    }
}
