use crate::cycle::Cycle;
use crate::field::Field;
use crate::field_coordinate::FieldCoordinate;
use crate::player_symbol::PlayerSymbol;
use crate::DEFAULT_BOARD_SIZE;
use array2d::Array2D;
use petgraph::algo::astar;
use petgraph::data::{Element, FromElements};
use petgraph::stable_graph::NodeIndex;
use petgraph::visit::NodeIndexable;
use petgraph::{Graph, Undirected};
use std::collections::HashSet;
use std::iter;
use std::iter::zip;

struct Board {
    size: usize,
    positions: Array2D<Field>,
    connections: Graph<(), usize, Undirected>,
    last_cycle: Option<Cycle>,
}

impl Default for Board {
    fn default() -> Self {
        Board::new(DEFAULT_BOARD_SIZE)
    }
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            size,
            positions: Array2D::filled_with(Field::Entangled(vec![None; size * size]), size, size),
            connections: Graph::from_elements(iter::repeat_n(
                Element::Node { weight: () },
                size * size,
            )),
            last_cycle: None,
        }
    }

    #[allow(unused)]
    pub fn mark(
        &mut self,
        fields_coordinates: Vec<FieldCoordinate>,
        player_symbol: PlayerSymbol,
        turn: usize,
    ) -> Result<Option<Cycle>, ()> {
        let mut hash_set = HashSet::<FieldCoordinate>::new();
        let mut fields = fields_coordinates
            .iter()
            .map(|&field_coordinate| {
                let field = self.positions.get(field_coordinate.y, field_coordinate.x);
                if hash_set.contains(&field_coordinate) {
                    return Err(());
                } else {
                    hash_set.insert(field_coordinate);
                }
                match field {
                    Some(field) => {
                        if !matches!(field, &Field::Entangled(_)) {
                            return Err(());
                        }
                        Ok(field.clone())
                    }
                    None => Err(()),
                }
            })
            .try_collect::<Vec<Field>>()?;

        zip(
            fields.iter_mut().filter_map(|field| match field {
                Field::Entangled(value) => {
                    value[turn] = Some(player_symbol);
                    Some(Field::Entangled(value.to_owned()))
                }
                _ => None,
            }),
            &fields_coordinates,
        )
        .for_each(|(field, &field_coordinate)| {
            self.positions
                .set(field_coordinate.y, field_coordinate.x, field)
                .unwrap()
        });

        let nodes = fields_coordinates
            .iter()
            .map(|&field_coordinate| {
                self.connections
                    .from_index(FieldCoordinate::into_usize(&field_coordinate, self.size))
            })
            .collect::<Vec<_>>();

        let path = astar(
            &self.connections,
            nodes[0],
            |finish| finish == nodes[1],
            |_| 1,
            |_| 0,
        );
        if path.is_some() {
            self.last_cycle = self.map_cycle(path, turn);
            Ok(self.last_cycle.clone())
        } else {
            self.connections.add_edge(nodes[0], nodes[1], turn);
            Ok(None)
        }
    }

    #[allow(unused)]
    pub fn collapse(
        &self,
        field_coordinate: FieldCoordinate,
        index: usize,
        player_symbol: PlayerSymbol,
    ) -> Result<(), ()> {
        todo!()
    }
    fn map_cycle(&self, cycle: Option<(i32, Vec<NodeIndex>)>, turn: usize) -> Option<Cycle> {
        let cycle = cycle.unwrap();
        let cycle_size = cycle.0;
        let cycle = cycle.1;
        let mut fields_indexes = vec![Vec::<usize>::new(); cycle_size as usize + 1];
        let fields_coordinates = cycle
            .iter()
            .map(|node_index| FieldCoordinate::from_usize(node_index.index(), self.size))
            .collect::<Vec<_>>();
        for i in 0..cycle_size {
            let weight = self
                .connections
                .edge_weight(
                    self.connections
                        .find_edge(cycle[i as usize], cycle[i as usize + 1])
                        .unwrap(),
                )
                .unwrap();
            fields_indexes[i as usize].push(*weight);
            fields_indexes[i as usize + 1].push(*weight);
        }
        fields_indexes[cycle.first().unwrap().index()].push(turn);
        fields_indexes[cycle.last().unwrap().index()].push(turn);
        Some(Cycle::new(fields_coordinates, fields_indexes))
    }
}

#[cfg(test)]
mod tests {
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
}
