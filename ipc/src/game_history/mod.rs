mod game_history_error;

use crate::from_server::board_ipc::Board;
use crate::game_history::game_history_error::GameHistoryError;
use crate::moves_history::MovesHistory;
use engine::game::Game;
use engine::player_move::Move;
use engine::player_symbol::PlayerSymbol;
use error_stack::Report;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GameHistory {
    game_uuid: Uuid,
    moves: Vec<(Move, PlayerSymbol)>,
    durations: Vec<Duration>,
    boards: Vec<Board>,
}

impl TryFrom<&MovesHistory> for GameHistory {
    type Error = Report<GameHistoryError>;
    fn try_from(value: &MovesHistory) -> Result<Self, Self::Error> {
        let mut game = Game::new(value.board_size());
        let boards = value
            .moves()
            .iter()
            .map(|(player_move, player)| {
                if let Err(err) = game.player_move(*player_move, *player) {
                    return Err(err.change_context(GameHistoryError {}));
                }
                Ok(Board::from(game.get_board()))
            })
            .try_collect::<Vec<_>>()?;
        Ok(Self {
            game_uuid: value.game_uuid(),
            moves: value.moves().clone(),
            durations: value.durations().clone(),
            boards,
        })
    }
}
