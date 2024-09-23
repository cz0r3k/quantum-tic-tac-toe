use crate::player_assignment::PlayerAssignment;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum FromServer {
    PONG,
    GameCreated(Uuid),
    PlayerAssignment(PlayerAssignment),
    GameNotCreated,
    GameAlreadyCreated,
}
