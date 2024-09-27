use core::fmt;
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct GameHistoryError {}

impl fmt::Display for GameHistoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("Game history Error")
    }
}
impl Error for GameHistoryError {}
