use crate::game_manager::GameManager;
use crate::game_repository::GameRepository;
use crate::process_tcp_connection::io;
use crate::server_error::ServerError;
use engine::game::game_error;
use engine::player_move::Move;
use engine::player_symbol::PlayerSymbol;
use error_stack::{bail, Result};
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
    game_manager: &mut Option<GameManager>,
    game_configuration: GameConfiguration,
    game_repository: Arc<Mutex<Box<Repository>>>,
    mut writer: Writer,
) -> Result<(), ServerError> {
    if game_manager.is_some() {
        error!("Game is already created");
        io::write_message(&mut writer, &FromServer::GameAlreadyCreated).await?;
        Ok(())
    } else {
        info!("{game_configuration:?}");
        let (game_manager_created, uuid) =
            create_new_game(game_configuration, game_repository.clone()).await;
        *game_manager = game_manager_created;
        info!("Game created: {uuid}",);
        io::write_message(&mut writer, &FromServer::GameCreated(uuid)).await?;
        io::write_message(
            &mut writer,
            &FromServer::Board(
                game_manager
                    .as_ref()
                    .expect("Game manager should exist")
                    .get_board(),
            ),
        )
        .await?;
        Ok(())
    }
}

pub async fn handle_get_player_assignment<Writer: AsyncWrite + Unpin>(
    game_manager: &Option<GameManager>,
    mut writer: Writer,
) -> Result<(), ServerError> {
    io::write_message(
        &mut writer,
        &FromServer::PlayerAssignment(
            game_manager
                .as_ref()
                .expect("Game manager should exist")
                .player_assignment(),
        ),
    )
    .await?;

    Ok(())
}

pub async fn handle_make_move<Writer: AsyncWrite + Unpin>(
    game_manager: &mut Option<GameManager>,
    mut writer: Writer,
    player_symbol: PlayerSymbol,
    player_move: Move,
) -> Result<(), ServerError> {
    match game_manager
        .as_mut()
        .expect("Game manager should exist")
        .make_move(player_symbol, player_move)
    {
        Ok(ref result) => {
            io::write_message(&mut writer, &FromServer::MoveOk(result.into())).await?;
            io::write_message(
                &mut writer,
                &FromServer::Board(
                    game_manager
                        .as_ref()
                        .expect("Game manager should exist")
                        .get_board(),
                ),
            )
            .await?;
        }
        Err(err) => {
            if err.current_context() == &game_error::GameError::MakingMoveError {
                io::write_message(&mut writer, &FromServer::GameCrash).await?;
                bail!(err.change_context(ServerError::GameError))
            }
            error!("{:?}", err);
            io::write_message(
                &mut writer,
                &FromServer::MoveErr(err.current_context().into()),
            )
            .await?;
        }
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

pub async fn check_is_game_created<Writer: AsyncWrite + Unpin>(
    game_manager: &Option<GameManager>,
    mut writer: Writer,
) -> Result<bool, ServerError> {
    if game_manager.is_none() {
        error!("Game is not created");
        io::write_message(&mut writer, &FromServer::GameNotCreated).await?;
        Ok(false)
    } else {
        Ok(true)
    }
}
