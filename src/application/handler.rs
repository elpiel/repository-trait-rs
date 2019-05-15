use crate::domain::channel::{Channel, ChannelRepository};
use crate::domain::RepositoryError;

pub struct Handler<'a> {
    channel_repository: &'a dyn ChannelRepository
}

impl<'a> Handler<'a> {
    pub fn new(channel_repository: &'a ChannelRepository) -> Self {
        Self { channel_repository }
    }

    pub async fn insert(&self, channels: &'a [Channel]) -> Result<(), RepositoryError> {
        for channel in channels.to_owned() {
            let _ = await!(self.channel_repository.insert(channel));
        }

        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<Channel>, RepositoryError> {
        await!(self.channel_repository.list())
    }
}