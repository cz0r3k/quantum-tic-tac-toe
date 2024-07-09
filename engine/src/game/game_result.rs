use crate::cycle::Cycle;
use crate::player_symbol::PlayerSymbol;

#[derive(Debug, PartialEq)]
pub enum GameResult {
    NextTurn,
    TurnAfterCollapse,
    NextTurnCycle(Cycle),
    GameEnd(Option<PlayerSymbol>),
}
