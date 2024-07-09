use crate::field_coordinate::FieldCoordinate;
use serde::Deserialize;

#[derive(Deserialize, Debug, Copy, Clone)]
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
