use crate::field_coordinate::FieldCoordinate;
use core::fmt;
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct BoardError {
    field_coordinate: FieldCoordinate,
}

impl BoardError {
    pub fn new(field_coordinate: FieldCoordinate) -> Self {
        BoardError { field_coordinate }
    }
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Board error (wrong coordinates x:{} y:{})",
            self.field_coordinate.x, self.field_coordinate.y
        )
    }
}

impl Error for BoardError {}
