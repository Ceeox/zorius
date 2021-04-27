use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    guard::Guard,
    Context, Error, Object, Result,
};
use bson::{de::from_document, oid::ObjectId};
use bson::{doc, to_document};
use futures::stream::StreamExt;
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument};

use crate::{
    api::database2,
    models::{
        merchandise::intern_merchandise::{
            InternMerchandise, InternMerchandiseId, InternMerchandiseStatus,
            InternMerchandiseUpdate, NewMerchandiseIntern,
        },
        roles::{Role, RoleGuard},
    },
};

use super::{claim::Claim, database, MDB_COLL_INTERN_MERCH};

#[derive(Default)]
pub struct InternMerchandiseQuery;

#[Object]
impl InternMerchandiseQuery {
    async fn get_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> Result<Option<InternMerchandise>> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_INTERN_MERCH);
        let filter = doc! {"_id": id};
        match collection.find_one(filter, None).await? {
            None => Ok(None),
            Some(doc) => Ok(Some(from_document::<InternMerchandise>(doc)?)),
        }
    }

    async fn get_by_merch_id(
        &self,
        ctx: &Context<'_>,
        merchandise_id: i32,
    ) -> Result<Option<InternMerchandise>> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_INTERN_MERCH);
        let filter = doc! {"merchandise_id": merchandise_id};
        match collection.find_one(filter, None).await? {
            None => Ok(None),
            Some(doc) => Ok(Some(from_document::<InternMerchandise>(doc)?)),
        }
    }

    async fn list(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, InternMerchandise, EmptyFields, EmptyFields>> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_INTERN_MERCH);
        let doc_count = collection.estimated_document_count(None).await? as usize;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = after.map(|after| after + 1).unwrap_or(0);
                let mut end = before.unwrap_or(doc_count);

                if let Some(first) = first {
                    end = (start + first).min(end);
                }
                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last };
                }
                let options = FindOptions::builder()
                    .skip(start as i64)
                    .limit(end as i64)
                    .build();
                let cursor = collection.find(None, options).await?;

                let mut connection = Connection::new(start > 0, end < doc_count);
                connection
                    .append_stream(cursor.enumerate().map(|(n, doc)| {
                        let merch = from_document::<InternMerchandise>(doc.unwrap()).unwrap();
                        Edge::with_additional_fields(n + start, merch, EmptyFields)
                    }))
                    .await;
                Ok(connection)
            },
        )
        .await
    }
}

#[derive(Default)]
pub struct InternMerchandiseMutation;

#[Object]
impl InternMerchandiseMutation {
    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::MerchandiseModerator")
    )))]
    async fn new(&self, ctx: &Context<'_>, new: NewMerchandiseIntern) -> Result<InternMerchandise> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_INTERN_MERCH);
        let new_merch = InternMerchandise::new(new);
        let doc = to_document(&new_merch)?;
        let _ = collection.insert_one(doc, None).await?;
        Ok(new_merch)
    }

    #[graphql(guard(race(
        RoleGuard(role = "Role::Admin"),
        RoleGuard(role = "Role::MerchandiseModerator")
    )))]
    async fn update_by_id(
        &self,
        ctx: &Context<'_>,
        id: InternMerchandiseId,
        update: InternMerchandiseUpdate,
    ) -> Result<Option<InternMerchandise>> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_INTERN_MERCH);
        let filter = doc! {"_id": id};
        let update = doc! {"$set": bson::to_bson(&update)?};
        let options = FindOneAndUpdateOptions::builder()
            .return_document(Some(ReturnDocument::After))
            .build();
        match collection
            .find_one_and_update(filter, update, Some(options))
            .await?
        {
            None => Ok(None),
            Some(doc) => Ok(Some(from_document(doc)?)),
        }
    }

    async fn change_status(
        &self,
        ctx: &Context<'_>,
        id: InternMerchandiseId,
        new_status: InternMerchandiseStatus,
    ) -> Result<Option<InternMerchandise>> {
        let _ = Claim::from_ctx(ctx)?;
        let collection = database(ctx)?.collection(MDB_COLL_INTERN_MERCH);
        let filter = doc! {"_id": id};
        let mut merch = match collection.find_one(filter, None).await? {
            None => return Ok(None),
            Some(doc) => from_document::<InternMerchandise>(doc)?,
        };

        let orderer_id = merch.orderer.clone();
        let user = match database2(ctx)?.get_user_by_id(orderer_id).await? {
            None => return Err(Error::new("the orderer could not be found")),
            Some(r) => r,
        };

        merch.change_status(new_status, user);

        Ok(None)
    }
}
