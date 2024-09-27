use core::fmt;
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum ServerError {
    RedisError,
    TCPError,
    SerializationError,
    GameError,
    SaveGameError,
    LocalHistoryError,
    HistoryNotExistError,
    RabbitMQError,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("Server error")
    }
}
impl Error for ServerError {}
