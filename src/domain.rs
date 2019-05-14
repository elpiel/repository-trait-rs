use futures_legacy::Future;

#[derive(Debug)]
pub enum RepositoryError {
    AlreadyExist,
}

pub type RepositoryFuture<T> = Box<Future<Item = T, Error=RepositoryError>  + Send>;

pub mod channel;