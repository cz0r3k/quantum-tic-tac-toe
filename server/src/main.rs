#![feature(duration_constructors)]
#![feature(async_closure)]

mod configuration;
mod game_manager;
mod game_repository;
mod player_enum;
mod process_tcp_connection;
mod server_error;
mod timer;

use crate::game_repository::redis_repository::RedisRepository;
use crate::process_tcp_connection::process;
use crate::server_error::ServerError;
use error_stack::ResultExt;
use log::{error, info};
use std::process;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    env_logger::init();
    let server_addr = "127.0.0.1";
    let server_port = "6379";
    let redis_connection_string = "redis://127.0.0.1/";
    let game_repository_redis = Arc::new(Mutex::new(RedisRepository::new(redis_connection_string)));

    match game_repository_redis.lock().await.connect().await {
        Ok(()) => {
            info!("connected to redis");
        }
        Err(err) => {
            error!("\n{:?}", err);
            process::exit(1);
        }
    }

    let tcp_listener = match TcpListener::bind(format!("{server_addr}:{server_port}"))
        .await
        .change_context(ServerError::TCPError)
        .attach_printable(format!("Can't bind socket {server_addr}:{server_port}"))
    {
        Ok(v) => {
            info!("Server is listening on: {server_addr}:{server_port}");
            v
        }
        Err(err) => {
            error!("\n{:?}", err);
            process::exit(1);
        }
    };

    loop {
        match tcp_listener
            .accept()
            .await
            .change_context(ServerError::TCPError)
            .attach_printable("Failed to accept a socket")
        {
            Ok((mut socket, socket_address)) => {
                let game_repository_redis = game_repository_redis.clone();
                tokio::spawn(async move {
                    info!("Accept new connection from: {socket_address}");
                    let (reader, writer) = socket.split();
                    process(reader, writer, game_repository_redis).await;
                });
            }
            Err(err) => {
                error!("\n{:?}", err);
            }
        }
    }
}
