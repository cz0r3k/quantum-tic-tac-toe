mod handle_message;
mod io;
#[cfg(test)]
mod test;

use crate::game_manager::GameManager;
use crate::game_repository::GameRepository;
use crate::process_tcp_connection::handle_message::{check_is_game_created, handle_end_game};
use ipc::to_server::ToServer;
use log::{error, info};
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::Mutex;

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
    if let Err(err) = loop {
        match io::read_message(&mut reader).await {
            Ok(message) => match message {
                ToServer::CreateGame(game_configuration) => {
                    info!("Create game");
                    if let Err(err) = handle_message::handle_create_game(
                        &mut game_manager,
                        game_configuration,
                        game_repository.clone(),
                        &mut writer,
                    )
                    .await
                    {
                        break Err(err);
                    }
                }
                ToServer::PING => {
                    info!("ping");
                    if let Err(err) = handle_message::handle_ping(&mut writer).await {
                        break Err(err);
                    }
                }
                ToServer::EndConnection => {
                    info!("End connection");
                    break Ok(());
                }
                ToServer::GetPlayerAssignment => {
                    info!("Get player assignment");
                    match check_is_game_created(&game_manager, &mut writer).await {
                        Ok(true) => {
                            if let Err(err) = handle_message::handle_get_player_assignment(
                                &game_manager,
                                &mut writer,
                            )
                            .await
                            {
                                break Err(err);
                            };
                        }
                        Ok(false) => continue,
                        Err(err) => break Err(err),
                    }
                }
                ToServer::MakeMove((player_symbol, player_move)) => {
                    info!("Player {player_symbol} move {player_move:?}");
                    match check_is_game_created(&game_manager, &mut writer).await {
                        Ok(true) => {
                            match handle_message::handle_make_move(
                                &mut game_manager,
                                &mut writer,
                                player_symbol,
                                player_move,
                            )
                            .await
                            {
                                Ok(true) => break Ok(()),
                                Ok(false) => continue,
                                Err(err) => break Err(err),
                            }
                        }
                        Ok(false) => continue,

                        Err(err) => break Err(err),
                    }
                }
                ToServer::EndGame(player_symbol) => {
                    match check_is_game_created(&game_manager, &mut writer).await {
                        Ok(true) => {
                            if let Err(err) =
                                handle_end_game(&mut game_manager, &mut writer, player_symbol).await
                            {
                                break Err(err);
                            }
                        }
                        Ok(false) => continue,
                        Err(err) => break Err(err),
                    }
                    break Ok(());
                }
            },
            Err(err) => break Err(err),
        }
    } {
        error!("{:?}", err);
    }
}
