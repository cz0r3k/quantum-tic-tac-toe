use serde::Serialize;

#[derive(Copy, Clone, PartialEq, Serialize)]
pub enum MoveType {
    Mark,
    Collapse,
}
