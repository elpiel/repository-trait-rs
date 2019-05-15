#![feature(await_macro, async_await)]

use std::net::SocketAddr;
use std::r#await;

use futures::compat::{Future01CompatExt};
use futures::future::{FutureExt, TryFutureExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use sentry::application::handler::Handler;
use sentry::domain::channel::Channel;
use sentry::domain::RepositoryError;
use sentry::infrastructure::persistence::channel::{MemoryChannelRepository, PostgresChannelRepository};
use sentry::infrastructure::persistence::DbPool;

fn spawn(pool: DbPool, mut _stream: TcpStream) {
    let pool = pool.clone();

    tokio::spawn(handle(pool).unit_error().boxed().compat());
}

async fn handle(pool: DbPool) {
    let response = await!(handle_request(pool)).unwrap();

    println!("{}", response);
}

async fn handle_request(pool: DbPool) -> Result<String, RepositoryError> {
    let postgres_channel_repository = PostgresChannelRepository::new(pool.clone());
    let initial_memory_channels = [Channel { id: "memory".to_owned() }];
//    println!("Initial memory channels: {:?}", &initial_memory_channels);
    let _memory_channel_repository = MemoryChannelRepository::new(Some(&initial_memory_channels));

//    let handler = Handler::new(&memory_channel_repository);
    let handler = Handler::new(&postgres_channel_repository);

    let channels = vec![
        Channel { id: "channel 1".to_owned() },
        Channel { id: "channel 2".to_owned() },
    ];

    println!("Insert some channels: {:?}", &channels);
    await!(handler.insert(&channels)).unwrap();

    let channels_list = await!(handler.list()).unwrap();

    Ok(format!("{:?}", channels_list))
}

fn main() {
    use std::env;

    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    // Bind the TCP listener
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", addr);

    tokio::run(
        bootstrap(listener).unit_error().boxed().compat()
    );
}

async fn bootstrap(listener: TcpListener) {
    let incoming = listener.incoming();

    let db_pool = await!(database_pool()).expect("Database connection failed");

    let fut = incoming
        .for_each(|stream| {
            spawn(db_pool.clone(), stream);

            Ok(())
        })
        .compat();

    await!(fut).unwrap();
}

async fn database_pool() -> Result<DbPool, tokio_postgres::Error> {
    let postgres_connection = bb8_postgres::PostgresConnectionManager::new(
        "postgresql://postgres:docker@localhost:5432/sentry",
        tokio_postgres::NoTls,
    );

    await!(bb8::Pool::builder().build(postgres_connection).compat())
}