use engine::cycle::Cycle;
use engine::game::game_result;
use engine::player_symbol::PlayerSymbol;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum GameResult {
    NextTurn,
    TurnAfterCollapse,
    NextTurnCycle(Cycle),
    GameEnd(Option<PlayerSymbol>),
}

impl From<&game_result::GameResult> for GameResult {
    fn from(value: &game_result::GameResult) -> Self {
        match value {
            game_result::GameResult::NextTurn => GameResult::NextTurn,
            game_result::GameResult::TurnAfterCollapse => GameResult::TurnAfterCollapse,
            game_result::GameResult::NextTurnCycle(v) => GameResult::NextTurnCycle(v.clone()),
            game_result::GameResult::GameEnd(v) => GameResult::GameEnd(*v),
        }
    }
}
