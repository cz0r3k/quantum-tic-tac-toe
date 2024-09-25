use array2d::Array2D;
use engine::field::Field;
use engine::{board, DEFAULT_BOARD_SIZE};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Board(pub Array2D<Field>);

impl From<&board::Board> for Board {
    fn from(value: &board::Board) -> Self {
        Board(value.get_positions())
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new(DEFAULT_BOARD_SIZE)
    }
}

impl Board {
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self(Array2D::filled_with(
            Field::Entangled(vec![None; size * size]),
            size,
            size,
        ))
    }
}
