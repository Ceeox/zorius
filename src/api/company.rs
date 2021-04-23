use async_graphql::{Context, MergedObject, Result};

use crate::models::company::Company;

#[derive(Default, MergedObject)]
pub struct CompanyQuery;

impl CompanyQuery {
    async fn get_company(&self, ctx: &Context<'_>) -> Result<Vec<Company>> {
        todo!();
    }

    async fn list_company(&self, ctx: &Context<'_>) -> Result<Vec<Company>> {
        todo!();
    }
}

#[derive(Default, MergedObject)]
pub struct CompanyMutation;

impl CompanyMutation {
    async fn new_company(&self, ctx: &Context<'_>) -> Result<Company> {
        todo!();
    }

    async fn update_company(&self, ctx: &Context<'_>) -> Result<Company> {
        todo!();
    }

    async fn delete_company(&self, ctx: &Context<'_>) -> Result<Vec<Company>> {
        todo!();
    }
}
