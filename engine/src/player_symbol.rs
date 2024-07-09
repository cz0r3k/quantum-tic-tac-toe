use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum PlayerSymbol {
    X,
    Y,
}

impl PlayerSymbol {
    #[allow(clippy::must_use_candidate)]
    pub fn opposite_symbol(player_symbol: PlayerSymbol) -> PlayerSymbol {
        match player_symbol {
            PlayerSymbol::Y => PlayerSymbol::X,
            PlayerSymbol::X => PlayerSymbol::Y,
        }
    }
}

impl fmt::Display for PlayerSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PlayerSymbol::X => write!(f, "X"),
            PlayerSymbol::Y => write!(f, "Y"),
        }
    }
}
