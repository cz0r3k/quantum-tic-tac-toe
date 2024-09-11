use crate::player_symbol::PlayerSymbol;

pub struct LinesResult {
    x: usize,
    o: usize,
}

impl Default for LinesResult {
    fn default() -> Self {
        Self::new()
    }
}

impl LinesResult {
    pub fn new() -> LinesResult {
        LinesResult { x: 0, o: 0 }
    }

    pub fn increase(&mut self, player_symbol: PlayerSymbol) {
        match player_symbol {
            PlayerSymbol::X => self.x += 1,
            PlayerSymbol::O => self.o += 1,
        }
    }

    pub fn is_full_line(&self) -> bool {
        self.x != 0 || self.o != 0
    }

    pub fn get_winner(&self) -> Option<PlayerSymbol> {
        if self.x > self.o {
            return Some(PlayerSymbol::X);
        }
        if self.o > self.x {
            return Some(PlayerSymbol::O);
        }
        None
    }
}
