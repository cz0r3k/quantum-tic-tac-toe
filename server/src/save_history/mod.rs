pub mod local_history;
pub mod rabbitmq_history;

use crate::server_error::ServerError;
use async_trait::async_trait;
use error_stack::Result;
use ipc::game_history::GameHistory;
use ipc::moves_history::MovesHistory;
use uuid::Uuid;

#[allow(unused)]
#[async_trait]
pub trait SaveHistory: Send {
    async fn save_game(&self, moves_history: &MovesHistory) -> Result<(), ServerError>;
    async fn get_game_history(&self, game_uuid: Uuid) -> Result<GameHistory, ServerError>;
}
