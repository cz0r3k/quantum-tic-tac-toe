pub mod mongodb;

use crate::history_error::HistoryError;
use async_trait::async_trait;
use error_stack::Result;
use ipc::game_history::GameHistory;
use ipc::moves_history::MovesHistory;
use uuid::Uuid;

#[async_trait]
#[allow(unused)]
trait HistoryManager {
    async fn save_game(&self, moves_history: &MovesHistory) -> Result<(), HistoryError>;
    async fn get_game_history(&self, game_uuid: Uuid) -> Result<GameHistory, HistoryError>;
}
