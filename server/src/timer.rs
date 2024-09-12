#![allow(unused)]
use tokio::time::Duration;
pub struct Timer {
    based_time: Duration,
    increment: Duration,
    player1_time: Duration,
    player2_time: Duration,
}
