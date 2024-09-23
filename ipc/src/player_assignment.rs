use crate::player_enum::Player;
use engine::player_symbol::PlayerSymbol;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct PlayerAssignment {
    player1: PlayerSymbol,
    player2: PlayerSymbol,
}

impl PlayerAssignment {
    #[must_use]
    pub fn new(first_player: Player) -> Self {
        match first_player {
            Player::Player1 => Self {
                player1: PlayerSymbol::X,
                player2: PlayerSymbol::O,
            },
            Player::Player2 => Self {
                player1: PlayerSymbol::O,
                player2: PlayerSymbol::X,
            },
        }
    }

    #[must_use]
    pub fn player1_symbol(&self) -> PlayerSymbol {
        self.player1
    }

    #[must_use]
    pub fn player2_symbol(&self) -> PlayerSymbol {
        self.player2
    }

    #[must_use]
    pub fn player_by_symbol(&self, symbol: PlayerSymbol) -> Player {
        if self.player1 == symbol {
            Player::Player1
        } else {
            Player::Player2
        }
    }
}
