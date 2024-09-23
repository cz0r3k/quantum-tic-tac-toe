mod handle_message;
mod io;
#[cfg(test)]
mod test;

use crate::game_manager::GameManager;
use crate::game_repository::GameRepository;
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
                    game_manager = match handle_message::handle_create_game(
                        game_manager,
                        game_configuration,
                        game_repository.clone(),
                        &mut writer,
                    )
                    .await
                    {
                        Ok(game_manager) => game_manager,
                        Err(err) => {
                            break Err(err);
                        }
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
                    if let Err(err) =
                        handle_message::handle_get_player_assignment(&game_manager, &mut writer)
                            .await
                    {
                        break Err(err);
                    };
                }
            },
            Err(err) => {
                break Err(err);
            }
        }
    } {
        error!("{:?}", err);
    }
}
