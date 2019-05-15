use futures::Future;
use std::pin::Pin;

#[derive(Debug)]
pub enum RepositoryError {
    AlreadyExist,
}

pub type RepositoryFuture<T> = Pin<Box<Future<Output=Result<T, RepositoryError>> + Send>>;

pub mod channel;