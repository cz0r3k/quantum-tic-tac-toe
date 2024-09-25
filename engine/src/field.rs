use crate::player_symbol::PlayerSymbol;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Debug, Deserialize)]
pub enum Field {
    Entangled(Vec<Option<PlayerSymbol>>),
    Collapsed(PlayerSymbol),
}
