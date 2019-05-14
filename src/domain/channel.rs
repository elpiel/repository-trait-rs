use futures::Future;

use crate::domain::{RepositoryFuture};

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: String,
}

pub trait ChannelRepository {
    fn list(&self) -> RepositoryFuture<Vec<Channel>>;
    fn insert(&self, channel: Channel) -> RepositoryFuture<()>;
}