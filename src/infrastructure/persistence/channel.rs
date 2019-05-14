use futures::future::{Future, TryFutureExt};
use futures_legacy::Future as LegacyFuture;

use crate::domain::{RepositoryError, RepositoryFuture};
use crate::domain::channel::{Channel, ChannelRepository};

pub struct MemoryChannelRepository {
    records: Vec<Channel>,
}

impl MemoryChannelRepository {
    pub fn new() -> Self {
        Self { records: vec![Channel { id: "memory".to_owned() }] }
    }
}

impl ChannelRepository for MemoryChannelRepository {
    fn list(&self) -> RepositoryFuture<Vec<Channel>> {
        Box::pin(
            futures::future::ok(
                self.records.iter().map(|channel| channel.clone()).collect()
            )
        )
    }
    fn insert(&self, channel: Channel) -> RepositoryFuture<()> {
        Box::pin(
            futures::future::ok(
                ()
            )
        )
    }
}