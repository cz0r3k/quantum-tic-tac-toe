#[cfg(test)]
mod test;

use crate::history_error::HistoryError;
use crate::history_manager::HistoryManager;
use async_trait::async_trait;
use error_stack::{report, Result, ResultExt};
use ipc::game_history::mongo_doc::GameHistoryMongoDoc;
use ipc::game_history::GameHistory;
use ipc::moves_history::MovesHistory;
use mongodb::bson::doc;
use mongodb::{Client, Collection};
use uuid::Uuid;

const DATABASE: &str = "game_history_service";
const COLLECTION: &str = "game_history";

struct MongodbHistory {
    collection: Collection<GameHistoryMongoDoc>,
}

#[async_trait]
impl HistoryManager for MongodbHistory {
    async fn save_game(&self, moves_history: &MovesHistory) -> Result<(), HistoryError> {
        let game_history =
            GameHistory::try_from(moves_history).change_context(HistoryError::MapGameHistory)?;
        let game_history = GameHistoryMongoDoc::from(game_history);
        self.collection
            .insert_one(game_history)
            .await
            .change_context(HistoryError::MongoDBError)?;
        Ok(())
    }

    async fn get_game_history(&self, game_uuid: Uuid) -> Result<GameHistory, HistoryError> {
        let doc = doc! {"game_uuid": game_uuid};
        let result = self
            .collection
            .find_one(doc)
            .await
            .change_context(HistoryError::MongoDBError)?;
        match result {
            Some(game_history) => Ok(GameHistory::from(game_history)),
            None => Err(report!(HistoryError::GameNotFound).attach_printable("Game not found")),
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
