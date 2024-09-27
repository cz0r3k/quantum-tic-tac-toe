use crate::save_history::SaveHistory;
use crate::server_error::ServerError;
use async_trait::async_trait;
use error_stack::{Result, ResultExt};
use ipc::game_history::GameHistory;
use ipc::moves_history::MovesHistory;
use tokio::fs::{read_to_string, File};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

const HISTORY_PATH: &str = "game_history";

#[allow(unused)]
pub struct Local {}

#[async_trait]
impl SaveHistory for Local {
    async fn save_game(&self, moves_history: &MovesHistory) -> Result<(), ServerError> {
        let game_history = GameHistory::try_from(moves_history)
            .change_context(ServerError::GameError)
            .attach_printable("Error creating game history")?;
        let mut file = File::create(format!("{HISTORY_PATH}/{}", moves_history.game_uuid()))
            .await
            .change_context(ServerError::SaveGameError)
            .attach_printable("Error creating file")?;

        let json_string = serde_json::to_string(&game_history)
            .change_context(ServerError::SaveGameError)
            .attach_printable("Error serialization to json")?;

        file.write_all(json_string.as_bytes())
            .await
            .change_context(ServerError::GameError)
            .attach_printable("Error writing to file")?;
        Ok(())
    }

    async fn get_game_history(&self, game_uuid: Uuid) -> Result<GameHistory, ServerError> {
        let json_string = read_to_string(format!("{HISTORY_PATH}/{game_uuid}"))
            .await
            .change_context(ServerError::HistoryNotExistError)
            .attach_printable("Error reading file")?;
        let game_history = serde_json::from_str(&json_string)
            .change_context(ServerError::SaveGameError)
            .attach_printable("Error deserialization from json")?;
        Ok(game_history)
    }
}
#[allow(unused)]
impl Local {
    pub fn new() -> Result<Self, ServerError> {
        std::fs::create_dir_all(HISTORY_PATH)
            .change_context(ServerError::LocalHistoryError)
            .attach_printable("Can't create directory")?;
        Ok(Self {})
    }
}
