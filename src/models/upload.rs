use async_graphql::{SimpleObject, ID};
use futures::lock::Mutex;
use slab::Slab;

pub type Storage = Mutex<Slab<FileInfo>>;
#[derive(Clone, SimpleObject)]
pub struct FileInfo {
    pub id: ID,
    pub filename: String,
    pub mimetype: Option<String>,
}
