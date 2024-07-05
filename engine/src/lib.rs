#![feature(iter_repeat_n)]
#![feature(iterator_try_collect)]

const DEFAULT_BOARD_SIZE: usize = 3;

mod board;
mod cycle;
mod field;
mod field_coordinate;
pub mod game;
mod player_symbol;
