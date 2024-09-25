use engine::player_move::Move;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct GameHistory {
    game_uuid: Uuid,
    moves: Vec<Move>,
    durations: Vec<Duration>,
}

impl GameHistory {
    pub fn add_move(&mut self, player_move: Move, duration: Duration) {
        self.moves.push(player_move);
        self.durations.push(duration);
    }

    #[must_use]
    pub fn new(game_uuid: Uuid, size: usize) -> Self {
        Self {
            game_uuid,
            moves: Vec::with_capacity(size + size / 2),
            durations: Vec::with_capacity(size + size / 2),
        }
    }
}
