#![feature(iter_repeat_n)]
#![feature(iterator_try_collect)]

const DEFAULT_BOARD_SIZE: usize = 3;

mod board;
mod cycle;
mod field;
mod field_coordinate;
pub mod game;
mod game_status;
mod move_type;
pub mod player_move;
pub mod player_symbol;
