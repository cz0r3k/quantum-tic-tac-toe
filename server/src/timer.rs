#![allow(unused)]
use tokio::time::Duration;
pub struct Timer {
    based_time: Duration,
    increment: Duration,
    player1_time: Duration,
    player2_time: Duration,
}

impl Timer {
    pub fn new(
        based_time: Duration,
        increment: Duration,
        player1_time: Duration,
        player2_time: Duration,
    ) -> Self {
        Self {
            based_time,
            increment,
            player1_time,
            player2_time,
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        let based_time = Duration::from_mins(5);
        Self::new(based_time, Duration::from_secs(1), based_time, based_time)
    }
}
