use crate::player_symbol::PlayerSymbol;
use serde::Serialize;

#[derive(Clone, PartialEq, Serialize, Debug)]
pub enum Field {
    Entangled(Vec<Option<PlayerSymbol>>),
    #[allow(unused)]
    Collapsed(PlayerSymbol),
}
