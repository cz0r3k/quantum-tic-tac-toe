use crate::player_enum::Player;
#[cfg(not(test))]
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_GAME_SIZE: usize = 3;
const DEFAULT_BASED_TIME: Duration = Duration::from_mins(5);
const DEFAULT_INCREMENT: Duration = Duration::from_secs(1);

#[derive(Serialize, Deserialize, Debug)]
pub struct GameConfiguration {
    size: usize,
    based_time: Duration,
    increment: Duration,
    first_player: Player,
}

impl Default for GameConfiguration {
    fn default() -> Self {
        Self::new(
            DEFAULT_GAME_SIZE,
            DEFAULT_BASED_TIME,
            DEFAULT_INCREMENT,
            Some(Player::Player1),
        )
    }
}

impl GameConfiguration {
    #[must_use]
    pub fn size(&self) -> usize {
        self.size
    }

    #[must_use]
    pub fn new(
        size: usize,
        based_time: Duration,
        increment: Duration,
        first_player: Option<Player>,
    ) -> Self {
        let first_player = if let Some(first_player) = first_player {
            first_player
        } else {
            #[cfg(not(test))]
            {
                let mut rng = rand::thread_rng();
                match rng.gen_range(0..=1) {
                    1 => Player::Player2,
                    _ => Player::Player1,
                }
            }
            #[cfg(test)]
            Player::Player1
        };
        Self {
            size,
            based_time,
            increment,
            first_player,
        }
    }

    #[must_use]
    pub fn based_time(&self) -> Duration {
        self.based_time
    }

    #[must_use]
    pub fn increment(&self) -> Duration {
        self.increment
    }

    #[must_use]
    pub fn first_player(&self) -> &Player {
        &self.first_player
    }
}
