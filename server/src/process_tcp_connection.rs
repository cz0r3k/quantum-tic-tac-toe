use crate::game_manager::GameManager;
use crate::game_repository::GameRepository;
use crate::server_error::ServerError;
use error_stack::{Result, ResultExt};
use ipc::from_server::FromServer;
use ipc::game_configuration::GameConfiguration;
use ipc::to_server::ToServer;
use log::{error, info};
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::Mutex;
use uuid::Uuid;

const BUFFER_SIZE: usize = 1024;

pub async fn process<
    Repository: GameRepository + ?Sized,
    Reader: AsyncRead + Unpin,
    Writer: AsyncWrite + Unpin,
>(
    mut reader: Reader,
    mut writer: Writer,
    game_repository: Arc<Mutex<Box<Repository>>>,
) {
    let mut game_manager: Option<GameManager> = None;
    loop {
        match read_message(&mut reader).await {
            Ok(message) => match message {
                ToServer::CreateGame(game_configuration) => {
                    if game_manager.is_some() {
                        error!("Game is already created");
                        continue;
                    }
                    info!("{game_configuration:?}");
                    let (game_manager_created, uuid) =
                        create_new_game(game_configuration, game_repository.clone()).await;
                    game_manager = game_manager_created;
                    info!("Game created: {uuid}",);
                    if let Err(e) = write_message(&mut writer, &FromServer::GameCreated(uuid)).await
                    {
                        error!("error writing to socket: {:?}", e);
                        break;
                    }
                }
                ToServer::PING => {
                    info!("ping");
                    if let Err(e) = write_message(&mut writer, &FromServer::PONG).await {
                        error!("error writing to socket: {:?}", e);
                        break;
                    }
                }
                ToServer::EndConnection => {
                    info!("connection is closed");
                    break;
                }
            },
            Err(e) => {
                error!("error reading socket: {:?}", e);
                break;
            }
        }
    }
}

async fn create_new_game<Repository: GameRepository + ?Sized>(
    game_configuration: GameConfiguration,
    game_repository: Arc<Mutex<Box<Repository>>>,
) -> (Option<GameManager>, Uuid) {
    #[cfg(test)]
    let mut uuid = Uuid::nil();
    #[cfg(not(test))]
    let mut uuid = Uuid::new_v4();
    while !game_repository.lock().await.add_game(uuid).await {
        uuid = Uuid::new_v4();
    }
    (Some(GameManager::new(uuid, &game_configuration)), uuid)
}

async fn read_message<Reader: AsyncRead + Unpin>(
    mut reader: Reader,
) -> Result<ToServer, ServerError> {
    let mut buf = [0; BUFFER_SIZE];
    let n = reader
        .read(&mut buf)
        .await
        .change_context(ServerError::TCPError)
        .attach_printable("Error reading")?;
    info!("read {n} bytes");
    if n == 0 {
        return Ok(ToServer::EndConnection);
    }
    let decoded = bincode::deserialize::<ToServer>(&buf[..n])
        .change_context(ServerError::SerializationError)
        .attach_printable("Can't be deserialized")?;
    Ok(decoded)
}

async fn write_message<Writer: AsyncWrite + Unpin>(
    mut writer: Writer,
    message: &FromServer,
) -> Result<(), ServerError> {
    let encode: Vec<u8> = bincode::serialize(&message)
        .change_context(ServerError::SerializationError)
        .attach_printable("Can't be serialized")?;
    let n = writer
        .write(&encode)
        .await
        .change_context(ServerError::TCPError)
        .attach_printable("Error writing")?;
    info!("write {n} bytes");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game_repository::local_repository::LocalRepository;

    #[tokio::test]
    async fn create_new_game() {
        let game_configuration = GameConfiguration::default();
        let repository = Arc::new(Mutex::new(Box::new(LocalRepository::new())));
        let reader = tokio_test::io::Builder::new()
            .read(&bincode::serialize(&ToServer::CreateGame(game_configuration)).unwrap())
            .build();
        let writer = tokio_test::io::Builder::new()
            .write(&bincode::serialize(&FromServer::GameCreated(Uuid::nil())).unwrap())
            .build();
        let () = process(reader, writer, repository).await;
    }
}
