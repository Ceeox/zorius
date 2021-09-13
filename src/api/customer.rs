use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Result,
};

use crate::{
    api::{claim::Claim, database},
    models::{
        customer::{Customer as DbCustomer, CustomerId},
        project::Project as DbProject,
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
        let customer = DbCustomer::get_customer_by_id(pool, id).await?;
        let mut projects = Some(DbProject::get_projects_for_customer_id(pool, id).await?);
        let mut res: Customer = customer.into();
        std::mem::swap(&mut res.projects, &mut projects);

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
        let count = DbCustomer::count_customers(pool).await? as usize;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = after
                    .map(|after: usize| after.saturating_add(1))
                    .unwrap_or(0);
                let mut end = before.unwrap_or(count);

                if let Some(first) = first {
                    end = (start.saturating_add(first)).min(end);
                }
                if let Some(last) = last {
                    start = if last > end.saturating_sub(start) {
                        end
                    } else {
                        end.saturating_sub(last)
                    };
                }
                let limit = (end.saturating_sub(start)) as i64;

                let customers = DbCustomer::list_customer(pool, start as i64, limit).await?;

                let mut connection = Connection::new(start > 0, end < count);
                connection.append(
                    customers
                        .into_iter()
                        .enumerate()
                        .map(|(n, db_customer)| Edge::new(n + start, Customer::from(db_customer))),
                );
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

        Ok(DbCustomer::new(pool, new_customer).await?.into())
    }

    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::WorkReportModerator")
    // )))]
    async fn update_customer(
        &self,
        ctx: &Context<'_>,
        _id: CustomerId,
        _update: UpdateCustomer,
    ) -> Result<Customer> {
        let _ = Claim::from_ctx(ctx)?;
        let _pool = database(ctx)?.get_pool();

        todo!()
    }

    // #[graphql(guard(race(
    //     RoleGuard(role = "Role::Admin"),
    //     RoleGuard(role = "Role::MerchandiseModerator")
    // )))]
    async fn delete_customer(&self, ctx: &Context<'_>, id: CustomerId) -> Result<Customer> {
        let _ = Claim::from_ctx(ctx)?;
        let pool = database(ctx)?.get_pool();

        Ok(DbCustomer::delete_customer(pool, id).await?.into())
    }
}
