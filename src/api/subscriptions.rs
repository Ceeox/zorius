use std::time::Duration;

use async_graphql::Subscription;
use futures::stream::{Stream, StreamExt};

pub struct RootSubscription;

#[Subscription]
impl RootSubscription {
    async fn integers(&self, #[graphql(default = "1")] step: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio::time::interval(Duration::from_secs(1)).map(move |_| {
            value += step;
            value
        })
    }
}
