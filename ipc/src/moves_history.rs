use engine::player_move::Move;
use engine::player_symbol::PlayerSymbol;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MovesHistory {
    game_uuid: Uuid,
    board_size: usize,
    moves: Vec<(Move, PlayerSymbol)>,
    durations: Vec<Duration>,
}

impl MovesHistory {
    pub fn add_move(&mut self, player_move: Move, duration: Duration, player: PlayerSymbol) {
        self.moves.push((player_move, player));
        self.durations.push(duration);
    }

    #[must_use]
    pub fn new(game_uuid: Uuid, size: usize) -> Self {
        Self {
            game_uuid,
            board_size: size,
            moves: Vec::with_capacity(size + size / 2),
            durations: Vec::with_capacity(size + size / 2),
        }
    }

    #[must_use]
    pub fn game_uuid(&self) -> Uuid {
        self.game_uuid
    }

    pub(super) fn board_size(&self) -> usize {
        self.board_size
    }

    pub(super) fn moves(&self) -> &Vec<(Move, PlayerSymbol)> {
        &self.moves
    }

    pub(super) fn durations(&self) -> &Vec<Duration> {
        &self.durations
    }
}
