use super::*;
use crate::field_coordinate::FieldCoordinate;

#[test]
fn wrong_player_turn() {
    let mut game = Game::new(3);
    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 1, y: 0 },
    };

    let result = game.player_move(player_move, PlayerSymbol::Y).unwrap_err();
    assert_eq!(result.current_context(), &GameError::PlayerTurnError);
}

#[test]
fn wrong_move_type() {
    let mut game = Game::new(3);
    let player_move = Move::Collapse {
        field: FieldCoordinate { x: 0, y: 0 },
        index: 0,
    };

    let result = game.player_move(player_move, PlayerSymbol::X).unwrap_err();
    assert_eq!(result.current_context(), &GameError::MoveTypeError);
}

#[test]
fn next_turn() {
    let mut game = Game::new(3);
    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 1, y: 0 },
    };
    let result = game.player_move(player_move, PlayerSymbol::X).unwrap();
    assert_eq!(result, GameResult::NextTurn);
    assert_eq!(game.game_status.get_turn(), 1);
}
