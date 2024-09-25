use crate::game_configuration::GameConfiguration;
use engine::player_move::Move;
use engine::player_symbol::PlayerSymbol;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ToServer {
    CreateGame(GameConfiguration),
    PING,
    EndConnection,
    GetPlayerAssignment,
    MakeMove((PlayerSymbol, Move)),
    EndGame(Option<PlayerSymbol>),
}
