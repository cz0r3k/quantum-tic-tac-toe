#![feature(iterator_try_collect)]

pub const DEFAULT_BOARD_SIZE: usize = 3;

pub mod board;
pub mod cycle;
pub mod field;
mod field_coordinate;
pub mod game;
mod game_status;
mod move_type;
pub mod player_move;
pub mod player_symbol;
