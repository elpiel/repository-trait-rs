use std::sync::{Arc, Mutex};

use futures::future::{Future, TryFutureExt};
use futures_legacy::Future as LegacyFuture;

use crate::domain::{RepositoryError, RepositoryFuture};
use crate::domain::channel::{Channel, ChannelRepository};
use crate::infrastructure::persistence::DbPool;

pub struct MemoryChannelRepository {
    records: Arc<Mutex<Vec<Channel>>>,
}

impl MemoryChannelRepository {
    pub fn new(initial_channels: Option<&[Channel]>) -> Self {
        Self { records: Arc::new(Mutex::new(initial_channels.unwrap_or(&[]).to_vec())) }
    }
}

impl ChannelRepository for MemoryChannelRepository {
    fn list(&self) -> RepositoryFuture<Vec<Channel>> {
        Box::pin(
            futures::future::ok(
                self.records.lock().unwrap().iter().map(|channel| channel.clone()).collect()
            )
        )
    }

    fn insert(&self, channel: Channel) -> RepositoryFuture<()> {
        &self.records.lock().unwrap().push(channel);

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