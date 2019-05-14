use crate::domain::channel::{Channel, ChannelRepository};
use crate::domain::RepositoryError;

pub struct Handler<'a> {
    channel_repository: &'a dyn ChannelRepository
}

impl<'a> Handler<'a> {
    pub fn new(channel_repository: &'a ChannelRepository) -> Self {
        Self { channel_repository }
    }

    pub async fn list(&self) -> Result<Vec<Channel>, RepositoryError> {
        let channels = vec![
            Channel { id: "channel 1".to_owned() },
            Channel { id: "channel 2".to_owned() },
        ];

        for channel in channels {
            await!(self.channel_repository.insert(channel));
        }

        await!(self.channel_repository.list())
    }
}