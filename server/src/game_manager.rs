use crate::timer::Timer;
use engine::game::game_error::GameError;
use engine::game::game_result::GameResult;
use engine::game::Game;
use engine::player_move::Move;
use engine::player_symbol::PlayerSymbol;
use error_stack::Result;
use ipc::from_server::board_ipc::Board;
use ipc::game_configuration::GameConfiguration;
use ipc::game_history::GameHistory;
use ipc::player_assignment::PlayerAssignment;
use std::time::Duration;
use uuid::Uuid;

pub struct GameManager {
    #[allow(unused)]
    uuid: Uuid,
    game: Game,
    #[allow(unused)]
    timer: Timer,
    player_assignment: PlayerAssignment,
    history: GameHistory,
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
            history: GameHistory::new(uuid, game_configuration.size()),
        }
    }

    pub fn player_assignment(&self) -> PlayerAssignment {
        self.player_assignment
    }

    pub fn make_move(
        &mut self,
        player: PlayerSymbol,
        player_move: Move,
    ) -> Result<GameResult, GameError> {
        let result = self.game.player_move(player_move, player)?;
        self.history.add_move(player_move, Duration::default());
        Ok(result)
    }

    pub fn get_board(&self) -> Board {
        self.game.get_board().into()
    }
}
