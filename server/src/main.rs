#![feature(duration_constructors)]

mod configuration;
mod game_manager;
mod game_repository;
mod player_enum;
mod process_tcp_connection;
mod server_error;
mod timer;

use crate::configuration::Configuration;
use crate::process_tcp_connection::process;
use crate::server_error::ServerError;
use error_stack::ResultExt;
use log::{error, info};
use std::process;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();

    configuration.set_env_logger();

    info!("{:?}", configuration);

    let game_repository = match configuration.create_game_repository().await {
        Ok(game_repository) => game_repository,
        Err(e) => {
            error!("{:?}", e);
            process::exit(1);
        }
    };

    let tcp_listener = match TcpListener::bind(configuration.server_address())
        .await
        .change_context(ServerError::TCPError)
        .attach_printable(format!(
            "Can't bind socket {}",
            configuration.server_address()
        )) {
        Ok(listener) => {
            info!("Server is listening on: {}", configuration.server_address());
            listener
        }
        Err(err) => {
            error!("{:?}", err);
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
