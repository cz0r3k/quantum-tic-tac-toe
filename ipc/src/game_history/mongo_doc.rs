use crate::from_server::board_ipc::Board;
use crate::game_history::GameHistory;
use bson::serde_helpers::uuid_1_as_binary;
use engine::player_move::Move;
use engine::player_symbol::PlayerSymbol;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GameHistoryMongoDoc {
    #[serde(with = "uuid_1_as_binary")]
    game_uuid: Uuid,
    moves: Vec<(Move, PlayerSymbol)>,
    durations: Vec<Duration>,
    boards: Vec<Board>,
}

impl From<GameHistory> for GameHistoryMongoDoc {
    fn from(value: GameHistory) -> Self {
        GameHistoryMongoDoc {
            game_uuid: value.game_uuid,
            moves: value.moves,
            durations: value.durations,
            boards: value.boards,
        }
    }
}

impl From<GameHistoryMongoDoc> for GameHistory {
    fn from(value: GameHistoryMongoDoc) -> Self {
        GameHistory {
            game_uuid: value.game_uuid,
            moves: value.moves,
            durations: value.durations,
            boards: value.boards,
        }
    }
}
