use super::*;

#[test]
fn empty_board_3x3() {
    let board = Board::new(3);
    assert_eq!(
        board.positions,
        Array2D::filled_with(Field::Entangled(vec![None; 9]), 3, 3)
    );
}

#[test]
fn default_board() {
    let board = Board::new(3);
    let default_board = Board::default();
    assert_eq!(board.positions, default_board.positions);
    assert_eq!(board.size, default_board.size);
}

#[test]
fn first_mark() {
    let mut board = Board::new(3);
    let fields_coordinates = vec![
        FieldCoordinate { x: 0, y: 0 },
        FieldCoordinate { x: 1, y: 0 },
    ];
    let _ = board.mark(fields_coordinates, PlayerSymbol::X, 0);
    let mut array = Array2D::filled_with(Field::Entangled(vec![None; 9]), 3, 3);
    let mut field = vec![Some(PlayerSymbol::X)];
    field.extend([None; 8]);
    array.set(0, 0, Field::Entangled(field.clone())).unwrap();
    array.set(0, 1, Field::Entangled(field)).unwrap();
    assert_eq!(board.positions, array);
}

#[test]
fn none_cycle() {
    let mut board = Board::new(3);
    let fields_coordinates = vec![
        FieldCoordinate { x: 0, y: 0 },
        FieldCoordinate { x: 1, y: 0 },
    ];
    assert!(board
        .mark(fields_coordinates, PlayerSymbol::X, 0)
        .unwrap()
        .is_none());
}

#[test]
fn mark_out_of_band() {
    let mut board = Board::new(3);
    let fields_coordinates = vec![
        FieldCoordinate { x: 3, y: 0 },
        FieldCoordinate { x: 3, y: 1 },
    ];
    assert!(board.mark(fields_coordinates, PlayerSymbol::X, 0).is_err())
}

#[test]
fn mark_on_collapsed() {
    let mut board = Board::new(3);
    board
        .positions
        .set(0, 0, Field::Collapsed(PlayerSymbol::X))
        .unwrap();
    let fields_coordinates = vec![
        FieldCoordinate { x: 0, y: 0 },
        FieldCoordinate { x: 0, y: 1 },
    ];
    assert!(board.mark(fields_coordinates, PlayerSymbol::X, 0).is_err())
}

#[test]
fn same_fields_coordinates() {
    let mut board = Board::new(3);
    board
        .positions
        .set(0, 0, Field::Collapsed(PlayerSymbol::X))
        .unwrap();
    let fields_coordinates = vec![
        FieldCoordinate { x: 0, y: 0 },
        FieldCoordinate { x: 0, y: 0 },
    ];
    assert!(board.mark(fields_coordinates, PlayerSymbol::X, 0).is_err())
}

#[test]
fn check_simple_cycle() {
    let mut board = Board::new(3);
    let fields_coordinates = vec![
        FieldCoordinate { x: 0, y: 0 },
        FieldCoordinate { x: 1, y: 0 },
    ];
    let _ = board.mark(fields_coordinates.clone(), PlayerSymbol::X, 0);
    assert!(board
        .mark(fields_coordinates, PlayerSymbol::Y, 1)
        .unwrap()
        .is_some());
}

#[test]
fn simply_cycle() {
    let mut board = Board::new(3);
    let fields_coordinates = vec![
        FieldCoordinate { x: 0, y: 0 },
        FieldCoordinate { x: 1, y: 0 },
    ];
    let _ = board.mark(fields_coordinates.clone(), PlayerSymbol::X, 0);
    let cycle = Some(Cycle::new(
        fields_coordinates.clone(),
        vec![vec![0, 1], vec![0, 1]],
    ));
    assert_eq!(
        cycle,
        board.mark(fields_coordinates, PlayerSymbol::Y, 1).unwrap()
    )
}

#[test]
#[ignore]
fn simply_collapsed_positions() {
    let mut board = Board::new(3);
    let fields_coordinates = vec![
        FieldCoordinate { x: 0, y: 0 },
        FieldCoordinate { x: 1, y: 0 },
    ];
    let _ = board.mark(fields_coordinates.clone(), PlayerSymbol::X, 0);
    let _ = board.mark(fields_coordinates.clone(), PlayerSymbol::Y, 1);
    let _ = board.collapse(FieldCoordinate { x: 0, y: 0 }, 0);

    let mut board_positions = vec![
        Field::Collapsed(PlayerSymbol::X),
        Field::Collapsed(PlayerSymbol::Y),
    ];
    board_positions.extend(vec![Field::Entangled(vec![None; 3 * 3]); 7]);
    let board_positions = Array2D::from_row_major(&board_positions, 3, 3).unwrap();
    assert_eq!(board.positions, board_positions);
}

#[test]
#[ignore]
fn simply_collapsed_connections() {
    let mut board = Board::new(3);
    let fields_coordinates = vec![
        FieldCoordinate { x: 0, y: 0 },
        FieldCoordinate { x: 1, y: 0 },
    ];
    let _ = board.mark(fields_coordinates.clone(), PlayerSymbol::X, 0);
    let _ = board.mark(fields_coordinates.clone(), PlayerSymbol::Y, 1);
    let _ = board.collapse(FieldCoordinate { x: 0, y: 0 }, 0);
    let connections: Graph<(), usize, Undirected> =
        Graph::from_elements(iter::repeat_n(Element::Node { weight: () }, 3 * 3));
    assert_eq!(connections.edge_count(), board.connections.edge_count());
}
