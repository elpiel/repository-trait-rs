#![feature(await_macro, async_await)]

use std::net::SocketAddr;

use tokio::await;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use sentry::application::handler::Handler;
use sentry::domain::channel::Channel;
use sentry::domain::RepositoryError;
use sentry::infrastructure::persistence::channel::{MemoryChannelRepository, PostgresChannelRepository};
use sentry::infrastructure::persistence::DbPool;

fn handle(pool: DbPool, mut stream: TcpStream) {
    let pool = pool.clone();

    tokio::spawn_async(async move {
        let channels = await!(handle_request(pool)).unwrap();

        println!("{:?}", channels);
    });
}

async fn handle_request(pool: DbPool) -> Result<Vec<Channel>, RepositoryError> {
    let initial_memory_channels = vec![Channel { id: "memory".to_owned() }];
    let channel_repository = MemoryChannelRepository::new(Some(&initial_memory_channels));
//    let channel_repository = PostgresChannelRepository::new(pool.clone());
    let handler = Handler::new(&channel_repository);

    await!(handler.list())
}

fn main() {
    use std::env;

    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    // Bind the TCP listener
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", addr);

    let manager = r2d2_postgres::PostgresConnectionManager::new(
        "postgresql://postgres:docker@localhost:5432/sentry".parse().unwrap(),
        postgres::NoTls,
    );

    let pool = r2d2::Pool::new(manager).unwrap();

    tokio::run_async(async move {
        let mut incoming = listener.incoming();

        while let Some(stream) = await!(incoming.next()) {
            let stream = stream.unwrap();
            handle(pool.clone(), stream);
        }
    });
}