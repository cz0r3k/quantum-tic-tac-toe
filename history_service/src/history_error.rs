use core::fmt;
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
#[allow(clippy::enum_variant_names, unused)]
pub enum HistoryError {
    MongoDBError,
    MapGameHistory,
    GameNotFound,
}

impl fmt::Display for HistoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("History service error")
    }
}
impl Error for HistoryError {}
