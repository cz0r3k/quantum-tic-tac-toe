pub mod board_ipc;
pub mod game_error_ipc;
pub mod game_result_ipc;

use crate::from_server::game_error_ipc::GameError;
use crate::player_assignment::PlayerAssignment;

use crate::from_server::board_ipc::Board;
use crate::from_server::game_result_ipc::GameResult;
use engine::player_symbol::PlayerSymbol;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum FromServer {
    PONG,
    GameCreated(Uuid),
    PlayerAssignment(PlayerAssignment),
    GameNotCreated,
    GameAlreadyCreated,
    Board(Board),
    MoveOk(GameResult),
    MoveErr(GameError),
    GameCrash,
    GameEnded(Option<PlayerSymbol>),
    EndOfTime(PlayerSymbol),
}
