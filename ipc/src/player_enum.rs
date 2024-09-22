#![allow(unused)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Player {
    Player1,
    Player2,
}
