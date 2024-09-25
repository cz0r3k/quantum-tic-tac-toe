use core::fmt;
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameError {
    PlayerTurnError,
    MoveTypeError,
    MoveAfterEnd,
    MakingMoveError,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("Game error")
    }
}

impl Error for GameError {}
