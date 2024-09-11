use crate::move_type::MoveType;
use crate::player_move::Move;
use crate::player_symbol::PlayerSymbol;

pub struct GameStatus {
    turn: usize,
    player_turn: PlayerSymbol,
    move_type: MoveType,
    is_end: bool,
    winner: Option<PlayerSymbol>,
}

impl Default for GameStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl GameStatus {
    pub fn new() -> Self {
        GameStatus {
            turn: 0,
            player_turn: PlayerSymbol::X,
            move_type: MoveType::Mark,
            is_end: false,
            winner: None,
        }
    }

    pub fn next_turn(&mut self, is_collapsed: bool) {
        if self.move_type == MoveType::Collapse {
            self.move_type = MoveType::Mark;
        } else {
            self.turn += 1;
            self.player_turn = PlayerSymbol::opposite_symbol(self.player_turn);
            if is_collapsed {
                self.move_type = MoveType::Collapse;
            }
        }
    }

    pub fn is_game_end(&self) -> bool {
        self.is_end
    }

    pub fn is_player_turn(&self, player_symbol: PlayerSymbol) -> bool {
        self.player_turn == player_symbol
    }

    pub fn is_good_move_type(&self, player_move: &Move) -> bool {
        match player_move {
            Move::Collapse { .. } => self.move_type == MoveType::Collapse,
            Move::Mark { .. } => self.move_type == MoveType::Mark,
        }
    }

    pub fn get_turn(&self) -> usize {
        self.turn
    }

    pub fn set_end(&mut self, winner: Option<PlayerSymbol>) {
        self.is_end = true;
        self.winner = winner;
    }
}
