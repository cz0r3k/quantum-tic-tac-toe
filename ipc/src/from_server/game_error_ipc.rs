use engine::game::game_error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum GameError {
    PlayerTurnError,
    MoveTypeError,
    MoveAfterEnd,
    MakingMoveError,
}

impl From<&game_error::GameError> for GameError {
    fn from(value: &game_error::GameError) -> Self {
        match value {
            game_error::GameError::PlayerTurnError => GameError::PlayerTurnError,
            game_error::GameError::MoveTypeError => GameError::MoveTypeError,
            game_error::GameError::MoveAfterEnd => GameError::MoveAfterEnd,
            game_error::GameError::MakingMoveError => GameError::MakingMoveError,
        }
    }
}
