use crate::player_enum::Player;
use crate::timer::Timer;
use engine::game::Game;
use uuid::Uuid;

#[allow(unused)]
pub struct GameManager {
    uuid: Uuid,
    game: Game,
    timer: Timer,
    first_player: Player,
}

impl GameManager {
    pub fn new(uuid: Uuid, size: usize) -> Self {
        Self {
            uuid,
            game: Game::new(size),
            timer: Timer::default(),
            first_player: Player::Player1,
        }
    }
}
