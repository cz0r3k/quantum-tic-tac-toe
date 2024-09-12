use super::*;
use crate::cycle::Cycle;
use crate::field_coordinate::FieldCoordinate;

#[test]
fn wrong_player_turn() {
    let mut game = Game::new(3);
    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 1, y: 0 },
    };

    let result = game.player_move(player_move, PlayerSymbol::O).unwrap_err();
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

#[test]
fn next_turn_cycle() {
    let mut game = Game::new(3);
    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 1, y: 0 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();
    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 1, y: 0 },
    };
    let result = game.player_move(player_move, PlayerSymbol::O).unwrap();
    let cycle = Cycle::new(
        vec![
            FieldCoordinate { x: 0, y: 0 },
            FieldCoordinate { x: 1, y: 0 },
        ],
        vec![vec![0, 1], vec![0, 1]],
    );
    assert_eq!(result, GameResult::NextTurnCycle(cycle));
    assert_eq!(game.game_status.get_turn(), 2);
}

#[test]
fn turn_after_collapse() {
    let mut game = Game::new(3);
    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 1, y: 0 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 1, y: 0 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::O).unwrap();
    assert_eq!(game.game_status.get_turn(), 2);

    let player_move = Move::Collapse {
        field: FieldCoordinate { x: 0, y: 0 },
        index: 0,
    };
    let result = game.player_move(player_move, PlayerSymbol::X).unwrap();
    assert_eq!(result, GameResult::TurnAfterCollapse);
    assert_eq!(game.game_status.get_turn(), 2);
}

#[test]
fn player_x_win() {
    let mut game = Game::new(3);
    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 1, y: 0 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 1 },
        field2: FieldCoordinate { x: 1, y: 1 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::O).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 2, y: 0 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 1 },
        field2: FieldCoordinate { x: 2, y: 1 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::O).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 1, y: 0 },
        field2: FieldCoordinate { x: 2, y: 0 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();

    let player_move = Move::Collapse {
        field: FieldCoordinate { x: 0, y: 0 },
        index: 0,
    };
    let result = game.player_move(player_move, PlayerSymbol::O).unwrap();

    assert_eq!(result, GameResult::GameEnd(Some(PlayerSymbol::X)));
}

#[test]
fn player_o_win() {
    let mut game = Game::new(3);
    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 1, y: 0 },
        field2: FieldCoordinate { x: 2, y: 0 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 0, y: 1 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::O).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 2, y: 0 },
        field2: FieldCoordinate { x: 2, y: 2 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 1 },
        field2: FieldCoordinate { x: 0, y: 2 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::O).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 1, y: 2 },
        field2: FieldCoordinate { x: 2, y: 2 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 0, y: 2 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::O).unwrap();

    let player_move = Move::Collapse {
        field: FieldCoordinate { x: 0, y: 0 },
        index: 1,
    };
    let result = game.player_move(player_move, PlayerSymbol::X).unwrap();

    assert_eq!(result, GameResult::GameEnd(Some(PlayerSymbol::O)));
}

#[test]
fn player_x_win_diagonal_size_4() {
    let mut game = Game::new(4);
    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 1, y: 1 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 1, y: 0 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::O).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 1, y: 1 },
        field2: FieldCoordinate { x: 2, y: 2 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 2, y: 0 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::O).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 2, y: 2 },
        field2: FieldCoordinate { x: 3, y: 3 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 3, y: 0 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::O).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 3, y: 3 },
        field2: FieldCoordinate { x: 0, y: 0 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();

    let player_move = Move::Collapse {
        field: FieldCoordinate { x: 0, y: 0 },
        index: 0,
    };
    let result = game.player_move(player_move, PlayerSymbol::O).unwrap();

    assert_eq!(result, GameResult::GameEnd(Some(PlayerSymbol::X)));
}

#[test]
fn draw() {
    let mut game = Game::new(3);
    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 0 },
        field2: FieldCoordinate { x: 1, y: 0 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 0, y: 1 },
        field2: FieldCoordinate { x: 1, y: 1 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::O).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 1, y: 0 },
        field2: FieldCoordinate { x: 2, y: 0 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 1, y: 1 },
        field2: FieldCoordinate { x: 2, y: 1 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::O).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 2, y: 0 },
        field2: FieldCoordinate { x: 2, y: 1 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::X).unwrap();

    let player_move = Move::Mark {
        field1: FieldCoordinate { x: 2, y: 0 },
        field2: FieldCoordinate { x: 2, y: 1 },
    };
    let _ = game.player_move(player_move, PlayerSymbol::O).unwrap();

    let player_move = Move::Collapse {
        field: FieldCoordinate { x: 2, y: 0 },
        index: 4,
    };
    let result = game.player_move(player_move, PlayerSymbol::X).unwrap();

    assert_eq!(result, GameResult::GameEnd(None));
}
