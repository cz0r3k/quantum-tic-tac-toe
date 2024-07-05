use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq, Hash, Eq)]
pub struct FieldCoordinate {
    pub x: usize,
    pub y: usize,
}

impl FieldCoordinate {
    pub fn into_usize(&self, size: usize) -> usize {
        self.y * size + self.x
    }

    pub fn from_usize(value: usize, size: usize) -> FieldCoordinate {
        FieldCoordinate {
            x: value % size,
            y: value / size,
        }
    }
}
