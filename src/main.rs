#![feature(await_macro, async_await)]

use std::net::SocketAddr;

use tokio::await;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use sentry::application::handler::Handler;
use sentry::domain::RepositoryError;
use sentry::domain::channel::{Channel, ChannelRepository};
use sentry::infrastructure::persistence::channel::MemoryChannelRepository;

fn handle(mut stream: TcpStream) {
    tokio::spawn_async(async move {
        let channels = await!(handle_request()).unwrap();

        println!("{:?}", channels);
    });
}

async fn handle_request() -> Result<Vec<Channel>, RepositoryError> {
    let channel_repository = MemoryChannelRepository::new();
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

    tokio::run_async(async {
        let mut incoming = listener.incoming();

        while let Some(stream) = await!(incoming.next()) {
            let stream = stream.unwrap();
            handle(stream);
        }
    });
}