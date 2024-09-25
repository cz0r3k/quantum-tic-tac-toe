#[allow(clippy::module_name_repetitions)]
pub mod game_error;
#[allow(clippy::module_name_repetitions)]
pub mod game_result;
#[cfg(test)]
mod test;

use crate::board::Board;
use crate::game::game_error::GameError;
use crate::game::game_result::GameResult;
use crate::game_status::GameStatus;
use crate::player_move::Move;
use crate::player_symbol::PlayerSymbol;

use error_stack::{Report, Result, ResultExt};

#[derive(Default)]
pub struct Game {
    board: Board,
    game_status: GameStatus,
}

impl Game {
    #[must_use]
    pub fn new(size: usize) -> Self {
        Game {
            board: Board::new(size),
            game_status: GameStatus::new(),
        }
    }

    /// # Errors
    ///
    /// Will return Err if game already end, not this player turn,
    /// wrong move type or wrong move coordinate or index.
    pub fn player_move(
        &mut self,
        player_move: Move,
        player_symbol: PlayerSymbol,
    ) -> Result<GameResult, GameError> {
        if self.game_status.is_game_end() {
            return Err(Report::new(GameError::MoveAfterEnd).attach_printable("Game already end"));
        }
        if !self.game_status.is_player_turn(player_symbol) {
            return Err(
                Report::new(GameError::PlayerTurnError).attach_printable("Not this player turn")
            );
        }
        if !self.game_status.is_good_move_type(&player_move) {
            return Err(Report::new(GameError::MoveTypeError).attach_printable("Wrong move type"));
        }

        match player_move {
            Move::Mark { field1, field2 } => {
                let cycle = self
                    .board
                    .mark(
                        &[field1, field2],
                        player_symbol,
                        self.game_status.get_turn(),
                    )
                    .change_context(GameError::MakingMoveError)?;
                self.game_status.next_turn(cycle.is_some());
                match cycle {
                    Some(cycle) => Ok(GameResult::NextTurnCycle(cycle)),
                    None => Ok(GameResult::NextTurn),
                }
            }
            Move::Collapse { field, index } => {
                self.board
                    .collapse(field, index)
                    .change_context(GameError::MakingMoveError)?;
                let (is_end, winner) = self.check_end();
                if is_end {
                    self.game_status.set_end(winner);
                    Ok(GameResult::GameEnd(winner))
                } else {
                    Ok(GameResult::TurnAfterCollapse)
                }
            }
        }
    }

    #[must_use]
    pub fn get_status(&self) -> &GameStatus {
        &self.game_status
    }

    #[must_use]
    pub fn get_board(&self) -> &Board {
        &self.board
    }

    /// Use this function if you want to end the game regardless of your position on the board
    ///
    /// # Errors
    ///
    /// Function will not return errors
    pub fn end_game(&mut self, winner: Option<PlayerSymbol>) -> Result<GameResult, GameError> {
        self.game_status.set_end(winner);
        Ok(GameResult::GameEnd(winner))
    }

    fn check_end(&self) -> (bool, Option<PlayerSymbol>) {
        let lines_result = self.board.check_all_lines();
        if lines_result.is_full_line() {
            (true, lines_result.get_winner())
        } else {
            (false, None)
        }
    }
}
