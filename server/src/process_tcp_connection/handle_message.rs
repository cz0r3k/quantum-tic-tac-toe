use crate::game_manager::GameManager;
use crate::game_repository::GameRepository;
use crate::process_tcp_connection::io;
use crate::server_error::ServerError;
use error_stack::Result;
use ipc::from_server::FromServer;
use ipc::game_configuration::GameConfiguration;
use log::{error, info};
use std::sync::Arc;
use tokio::io::AsyncWrite;
use tokio::sync::Mutex;
use uuid::Uuid;

pub async fn handle_ping<Writer: AsyncWrite + Unpin>(
    mut writer: Writer,
) -> Result<(), ServerError> {
    io::write_message(&mut writer, &FromServer::PONG).await?;
    Ok(())
}

pub async fn handle_create_game<Repository: GameRepository + ?Sized, Writer: AsyncWrite + Unpin>(
    game_manager: Option<GameManager>,
    game_configuration: GameConfiguration,
    game_repository: Arc<Mutex<Box<Repository>>>,
    mut writer: Writer,
) -> Result<Option<GameManager>, ServerError> {
    if game_manager.is_some() {
        error!("Game is already created");
        io::write_message(&mut writer, &FromServer::GameAlreadyCreated).await?;
        Ok(game_manager)
    } else {
        info!("{game_configuration:?}");
        let (game_manager, uuid) =
            create_new_game(game_configuration, game_repository.clone()).await;
        info!("Game created: {uuid}",);
        io::write_message(&mut writer, &FromServer::GameCreated(uuid)).await?;
        Ok(game_manager)
    }
}

pub async fn handle_get_player_assignment<Writer: AsyncWrite + Unpin>(
    game_manager: &Option<GameManager>,
    mut writer: Writer,
) -> Result<(), ServerError> {
    if let Some(ref game_manager) = game_manager {
        io::write_message(
            &mut writer,
            &FromServer::PlayerAssignment(game_manager.player_assignment()),
        )
        .await?;
    } else {
        io::write_message(&mut writer, &FromServer::GameNotCreated).await?;
    }
    Ok(())
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
