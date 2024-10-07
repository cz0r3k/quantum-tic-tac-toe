use crate::history_error::HistoryError;
use crate::history_manager::HistoryManager;
use async_trait::async_trait;
use error_stack::{bail, Result, ResultExt};
use ipc::game_history::GameHistory;
use ipc::moves_history::MovesHistory;
use mongodb::bson::doc;
use mongodb::{Client, Collection};
use uuid::Uuid;

#[allow(unused)]
const DATABASE: &str = "game_history_service";
#[allow(unused)]
const COLLECTION: &str = "game_history";

#[allow(unused)]
struct MongodbHistory {
    collection: Collection<GameHistory>,
}

#[async_trait]
impl HistoryManager for MongodbHistory {
    async fn save_game(&self, moves_history: &MovesHistory) -> Result<(), HistoryError> {
        let game_history =
            GameHistory::try_from(moves_history).change_context(HistoryError::MapGameHistory)?;
        self.collection.insert_one(game_history).await.change_context(HistoryError::MongoDBError)?;
        Ok(())
    }

    async fn get_game_history(&self, game_uuid: Uuid) -> Result<GameHistory, HistoryError> {
        let result = self.collection.find_one(
            doc! {"game_uuid": game_uuid.to_string()},
        ).await.change_context(HistoryError::MongoDBError)?;
        match result {
            Some(game_history) => Ok(game_history),
            None => bail!(HistoryError::GameNotFound),
        }
    }
}

#[allow(unused)]
impl MongodbHistory {
    async fn new(connection_string: &str) -> Result<Self, HistoryError> {
        let client = Client::with_uri_str(connection_string)
            .await
            .change_context(HistoryError::MongoDBError)?;
        Ok(MongodbHistory {
            collection: client.database(DATABASE).collection(COLLECTION),
        })
    }
}
