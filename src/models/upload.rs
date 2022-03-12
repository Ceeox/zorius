use async_graphql::SimpleObject;

#[derive(Clone, SimpleObject)]
pub struct FileInfo {
    pub filename: String,
}
