mod game_manager;
mod timer;

use log::{error, info};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    env_logger::init();
    let addr = "127.0.0.1";
    let port = "6379";
    match TcpListener::bind(format!("{addr}:{port}")).await {
        Ok(listener) => {
            info!("Server is listening on: {addr}:{port}");
            loop {
                match listener.accept().await {
                    Ok((socket, _)) => {
                        tokio::spawn(async move {
                            info!("Accept new connection");
                            process(socket).await;
                        });
                    }
                    Err(_) => {
                        error!("Failed to accept a socket");
                    }
                }
            }
        }
        Err(_) => {
            error!("Cannot bind socket");
        }
    }
}

#[allow(unused, clippy::unused_async)]
async fn process(socket: TcpStream) {
    todo!()
}
