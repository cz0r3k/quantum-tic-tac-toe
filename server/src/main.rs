#![feature(duration_constructors)]
#![feature(async_closure)]

mod configuration;
mod game_manager;
mod game_repository;
mod player_enum;
mod process_tcp_connection;
mod server_error;
mod timer;

use crate::configuration::Configuration;
use crate::game_repository::local_repository::LocalRepository;
use crate::game_repository::redis_repository::RedisRepository;
use crate::game_repository::{GameRepository, GameRepositoryEnum};
use crate::process_tcp_connection::process;
use crate::server_error::ServerError;
use error_stack::ResultExt;
use log::{error, info, LevelFilter};
use std::process;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    env_logger::builder()
        .filter_level(if configuration.debug() {
            LevelFilter::Debug
        } else {
            LevelFilter::Error
        })
        .init();
    info!("{:?}", configuration);

    let game_repository: Arc<Mutex<Box<dyn GameRepository>>> =
        Arc::new(Mutex::new(match configuration.game_repository() {
            GameRepositoryEnum::Redis(connection_string) => {
                let mut repository = RedisRepository::new(connection_string);
                info!("Connecting to redis: {connection_string}");
                if let Err(e) = repository.connect().await {
                    error!("{:?}", e);
                    process::exit(1);
                }
                info!("Connected to redis");
                Box::new(repository)
            }
            GameRepositoryEnum::Local => Box::new(LocalRepository::new()),
        }));

    let tcp_listener = match TcpListener::bind(configuration.server_address())
        .await
        .change_context(ServerError::TCPError)
        .attach_printable(format!(
            "Can't bind socket {}",
            configuration.server_address()
        )) {
        Ok(v) => {
            info!("Server is listening on: {}", configuration.server_address());
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
                let game_repository = game_repository.clone();
                tokio::spawn(async move {
                    info!("Accept new connection from: {socket_address}");
                    let (reader, writer) = socket.split();
                    process(reader, writer, game_repository).await;
                });
            }
            Err(err) => {
                error!("\n{:?}", err);
            }
        }
    }
}
