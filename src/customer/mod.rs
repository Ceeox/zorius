use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Subscription,
};
use futures::{stream, StreamExt};
use futures_util::Stream;
use uuid::Uuid;

use crate::{
    api::{database, MutationType},
    claim::Claim,
    errors::Result,
    guards::TokenGuard,
    simple_broker::SimpleBroker,
};

use self::{
    db::{count_customers, delete_customer, list_customers, new_customer, update_customer},
    model::{
        Customer, CustomerChanged, DbListOptions, ListCustomerOptions, NewCustomer, UpdateCustomer,
    },
};

mod db;
pub mod model;

#[derive(Default)]
pub struct CustomerQuery;

#[Object]
impl CustomerQuery {
    #[graphql(guard = "TokenGuard")]
    async fn customers(
        &self,
        ctx: &Context<'_>,
        options: Option<ListCustomerOptions>,
    ) -> async_graphql::Result<Connection<usize, Customer, EmptyFields, EmptyFields>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(ctx)?;
        let count = count_customers(db).await? as usize;
        let options = options.unwrap_or_default();
        let mut db_options = DbListOptions {
            ids: options.ids,
            ..Default::default()
        };

        query(
            options.after,
            options.before,
            options.first,
            options.last,
            |after, before, first, last| async move {
                let mut start = after.map(|after| after + 1).unwrap_or(0);
                let mut end = before.unwrap_or(count);
                if let Some(first) = first {
                    end = (start + first).min(end);
                }
                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last };
                }
                db_options.start = start as u64;
                db_options.limit = end as u64;

                let customers = match list_customers(db, db_options).await {
                    Ok(r) => r,
                    Err(_e) => return Err(async_graphql::Error::new("")),
                };

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
    #[graphql(guard = "TokenGuard")]
    async fn new_customer(&self, ctx: &Context<'_>, new: NewCustomer) -> Result<Option<Customer>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(ctx)?;

        if let Some(customer) = new_customer(db, new).await? {
            SimpleBroker::publish(CustomerChanged {
                mutation_type: MutationType::Created,
                id: customer.id,
            });
            return Ok(Some(customer.into()));
        }
        Ok(None)
    }

    #[graphql(guard = "TokenGuard")]
    async fn update_customer(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        update: UpdateCustomer,
    ) -> Result<Option<Customer>> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(ctx)?;

        if let Some(customer) = update_customer(db, id, update).await? {
            SimpleBroker::publish(CustomerChanged {
                mutation_type: MutationType::Updated,
                id: customer.id,
            });
            return Ok(Some(customer.into()));
        }
        Ok(None)
    }

    #[graphql(guard = "TokenGuard")]
    async fn delete_customer(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let _ = Claim::from_ctx(ctx)?;
        let db = database(ctx)?;
        let res = delete_customer(db, id).await? >= 1;
        SimpleBroker::publish(CustomerChanged {
            mutation_type: MutationType::Deleted,
            id,
        });

        Ok(res)
    }
}

#[derive(Debug, Default, Clone)]
pub struct CustomerSubscription;

#[Subscription]
impl CustomerSubscription {
    #[graphql(guard = "TokenGuard")]
    async fn customers(
        &self,
        mutation_type: Option<MutationType>,
    ) -> impl Stream<Item = CustomerChanged> {
        SimpleBroker::<CustomerChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}
