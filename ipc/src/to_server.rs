use crate::game_configuration::GameConfiguration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ToServer {
    CreateGame(GameConfiguration),
    PING,
    EndConnection,
}
