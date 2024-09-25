use crate::field_coordinate::FieldCoordinate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(tag = "type")]
pub enum Move {
    Mark {
        field1: FieldCoordinate,
        field2: FieldCoordinate,
    },
    Collapse {
        field: FieldCoordinate,
        index: usize,
    },
}
