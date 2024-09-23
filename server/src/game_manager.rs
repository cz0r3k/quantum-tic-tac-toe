use crate::timer::Timer;
use engine::game::Game;
use ipc::game_configuration::GameConfiguration;
use ipc::player_assignment::PlayerAssignment;
use uuid::Uuid;

#[allow(unused)]
pub struct GameManager {
    uuid: Uuid,
    game: Game,
    timer: Timer,
    player_assignment: PlayerAssignment,
}

impl GameManager {
    pub fn new(uuid: Uuid, game_configuration: &GameConfiguration) -> Self {
        Self {
            uuid,
            game: Game::new(game_configuration.size()),
            timer: Timer::new(
                game_configuration.based_time(),
                game_configuration.increment(),
            ),
            player_assignment: PlayerAssignment::new(*game_configuration.first_player()),
        }
    }

    pub fn player_assignment(&self) -> PlayerAssignment {
        self.player_assignment
    }
}
