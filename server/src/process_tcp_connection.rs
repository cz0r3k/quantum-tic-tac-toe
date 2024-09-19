use crate::game_manager::GameManager;
use crate::game_repository::GameRepository;
use crate::server_error::ServerError;
use error_stack::{bail, report, Result, ResultExt};
use ipc::from_server::FromServer;
use ipc::to_server::ToServer;
use log::{error, info};
use std::io;
use std::sync::Arc;
use tokio::io::Interest;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use uuid::Uuid;

pub async fn process<T: GameRepository>(socket: TcpStream, game_repository: Arc<Mutex<T>>) {
    let mut game_manager: Option<GameManager> = None;
    loop {
        match read_socket(&socket).await {
            Ok(message) => match message {
                ToServer::CreateGame => {
                    if game_manager.is_some() {
                        error!("Game is already created");
                        continue;
                    }
                    let (game_manager_created, uuid) =
                        create_new_game(3, game_repository.clone()).await;
                    game_manager = Some(game_manager_created);
                    info!("Game created: {uuid}",);
                    if let Err(e) = write_socket(&socket, &FromServer::GameCreated(uuid)).await {
                        if e.current_context() == &ServerError::TCPWouldBlockError {
                            continue;
                        }
                        error!("error writing to socket: {:?}", e);
                        break;
                    }
                }
                ToServer::Test => {
                    info!("test");
                }
                ToServer::EndConnection => {
                    info!("connection is closed");
                    break;
                }
            },
            Err(e) => {
                if e.current_context() == &ServerError::TCPWouldBlockError {
                    continue;
                }
                error!("error reading socket: {:?}", e);
                break;
            }
        }
    }
}

async fn write_socket(socket: &TcpStream, message: &FromServer) -> Result<(), ServerError> {
    let ready = socket
        .ready(Interest::WRITABLE)
        .await
        .change_context(ServerError::TCPError)
        .attach_printable("Can't check if socket is ready")?;
    let encode: Vec<u8> = bincode::serialize(&message)
        .change_context(ServerError::SerializationError)
        .attach_printable("Can't be serialized")?;
    if ready.is_writable() {
        match socket.try_write(&encode) {
            Ok(n) => {
                info!("write {n} bytes");
                Ok(())
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                bail!(ServerError::TCPWouldBlockError)
            }
            Err(e) => Err(report!(e)
                .change_context(ServerError::TCPError)
                .attach_printable("Error writing")),
        }
    } else {
        Err(report!(ServerError::TCPError).attach_printable("Socket is not writable"))
    }
}
async fn read_socket(socket: &TcpStream) -> Result<ToServer, ServerError> {
    let mut buf = vec![0; 1024];
    let ready = socket
        .ready(Interest::READABLE)
        .await
        .change_context(ServerError::TCPError)
        .attach_printable("Can't check if socket is ready")?;
    if ready.is_readable() {
        let n = match socket.try_read(&mut buf) {
            Ok(n) => n,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                bail!(ServerError::TCPWouldBlockError)
            }
            Err(e) => {
                return Err(report!(e)
                    .change_context(ServerError::TCPError)
                    .attach_printable("Error reading"));
            }
        };
        info!("read {n} bytes");
        if n == 0 {
            return Ok(ToServer::EndConnection);
        }
        let decoded = bincode::deserialize::<ToServer>(&buf[..n])
            .change_context(ServerError::SerializationError)
            .attach_printable("Can't be deserialized")?;
        Ok(decoded)
    } else {
        Err(report!(ServerError::TCPError).attach_printable("Socket is not readable"))
    }
}

async fn create_new_game<T: GameRepository>(
    size: usize,
    game_repository: Arc<Mutex<T>>,
) -> (GameManager, Uuid) {
    let mut uuid = Uuid::new_v4();
    while !game_repository.lock().await.add_game(uuid).await {
        uuid = Uuid::new_v4();
    }
    (GameManager::new(uuid, size), uuid)
}
