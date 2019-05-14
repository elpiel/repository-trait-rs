use futures::future::{Future, TryFutureExt};
use futures_legacy::Future as LegacyFuture;

use crate::domain::{RepositoryError, RepositoryFuture};
use crate::domain::channel::{Channel, ChannelRepository};
use crate::infrastructure::persistence::DbPool;

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


pub struct PostgresChannelRepository {
    pool: DbPool,
}

impl PostgresChannelRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl ChannelRepository for PostgresChannelRepository {
    fn list(&self) -> RepositoryFuture<Vec<Channel>> {
        let mut conn = self.pool.get().unwrap();
        let stmt = conn.prepare("SELECT channel_id FROM channels").unwrap();

        let results = conn
            .query(&stmt, &[])
            .unwrap()
            .iter()
            .map(|row| {
                Channel {
                    id: row.get("channel_id")
                }
            })
            .collect();

        Box::pin(
            futures::future::ok(
                results
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