use std::sync::{Arc, RwLock};

use futures::compat::Future01CompatExt;
use futures::future::FutureExt;
use futures_legacy::Future as LegacyFuture;
use futures_legacy::future::IntoFuture as LegacyIntoFuture;
use futures_legacy::stream::Stream as LegacyStream;
use try_future::try_future;

use crate::domain::{RepositoryError, RepositoryFuture};
use crate::domain::channel::{Channel, ChannelRepository};
use crate::infrastructure::persistence::DbPool;

pub struct MemoryChannelRepository {
    records: Arc<RwLock<Vec<Channel>>>,
}

impl MemoryChannelRepository {
    pub fn new(initial_channels: Option<&[Channel]>) -> Self {
        let memory_channels = initial_channels.unwrap_or(&[]).to_vec();

        Self { records: Arc::new(RwLock::new(memory_channels)) }
    }
}

impl ChannelRepository for MemoryChannelRepository {
    fn list(&self) -> RepositoryFuture<Vec<Channel>> {
        Box::pin(
            futures::future::ok(
                self.records.read().unwrap().iter().map(|channel| channel.clone()).collect()
            )
        )
    }

    fn insert(&self, channel: Channel) -> RepositoryFuture<()> {
        &self.records.write().unwrap().push(channel);

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
        let results = self.pool
            .run(|mut client| {
                client
                    .prepare("SELECT channel_id FROM channels")
                    .then(|res| match res {
                        Ok(stmt) => {
                            client
                                .query(&stmt, &[])
                                .collect()
                                .into_future()
                                .then(|res| match res {
                                    Ok(rows) => Ok((rows, client)),
                                    Err(err) => Err((err, client)),
                                })
                                .into()
                        }
                        Err(err) => try_future!(Err((err, client))),
                    })
                    .and_then(|(rows, client)| {
                        let channels = rows
                            .iter()
                            .map(|row| {
                                Channel {
                                    id: row.get("channel_id"),
                                }
                            })
                            .collect();

                        Ok((channels, client))
                    })
            })
            .map_err(|err| RepositoryError::AlreadyExist)
            .compat()
            .boxed();

        results
    }

    fn insert(&self, _channel: Channel) -> RepositoryFuture<()> {
        Box::pin(
            futures::future::ok(
                ()
            )
        )
    }
}