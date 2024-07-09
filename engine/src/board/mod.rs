use std::collections::HashSet;
use std::iter;
use std::iter::zip;

use array2d::Array2D;
use error_stack::{Report, Result};
use petgraph::algo::astar;
use petgraph::data::{Element, FromElements};
use petgraph::stable_graph::NodeIndex;
use petgraph::visit::NodeIndexable;
use petgraph::{Graph, Undirected};

use crate::board::board_error::BoardError;
use crate::cycle::Cycle;
use crate::field::Field;
use crate::field_coordinate::FieldCoordinate;
use crate::player_symbol::PlayerSymbol;
use crate::DEFAULT_BOARD_SIZE;

pub struct Board {
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

    pub fn mark(
        &mut self,
        fields_coordinates: &[FieldCoordinate],
        player_symbol: PlayerSymbol,
        turn: usize,
    ) -> Result<Option<Cycle>, BoardError> {
        let mut hash_set = HashSet::<FieldCoordinate>::new();
        let mut fields = fields_coordinates
            .iter()
            .map(|&field_coordinate| {
                let field = self.positions.get(field_coordinate.y, field_coordinate.x);
                if hash_set.contains(&field_coordinate) {
                    return Err(Report::new(BoardError::new(field_coordinate))
                        .attach_printable("Same coordinates"));
                }
                hash_set.insert(field_coordinate);
                match field {
                    Some(field) => {
                        if !matches!(field, &Field::Entangled(_)) {
                            return Err(Report::new(BoardError::new(field_coordinate))
                                .attach_printable("Filed is already entangled"));
                        }
                        Ok(field.clone())
                    }
                    None => Err(Report::new(BoardError::new(field_coordinate))
                        .attach_printable("Out of band coordinate")),
                }
            })
            .try_collect::<Vec<Field>>()?;

        zip(
            fields.iter_mut().filter_map(|field| match field {
                Field::Entangled(value) => {
                    value[turn] = Some(player_symbol);
                    Some(Field::Entangled(value.to_owned()))
                }
                Field::Collapsed(_) => None,
            }),
            fields_coordinates,
        )
        .for_each(|(field, &field_coordinate)| {
            self.positions
                .set(field_coordinate.y, field_coordinate.x, field)
                .unwrap();
        });

        let nodes = fields_coordinates
            .iter()
            .map(|&field_coordinate| {
                self.connections
                    .from_index(FieldCoordinate::into_usize(field_coordinate, self.size))
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
            self.last_cycle = Some(self.map_cycle(path, turn));
            Ok(self.last_cycle.clone())
        } else {
            self.connections.add_edge(nodes[0], nodes[1], turn);
            Ok(None)
        }
    }

    #[allow(unused)]
    pub fn collapse(
        &mut self,
        field_coordinate: FieldCoordinate,
        index: usize,
    ) -> Result<(), BoardError> {
        todo!()
    }
    #[allow(clippy::cast_sign_loss)]
    fn map_cycle(&self, cycle: Option<(i32, Vec<NodeIndex>)>, turn: usize) -> Cycle {
        let cycle = cycle.unwrap();
        let cycle_size = cycle.0 as usize;
        let cycle = cycle.1;
        let mut fields_indexes = vec![Vec::<usize>::new(); cycle_size + 1];
        let fields_coordinates = cycle
            .iter()
            .map(|node_index| FieldCoordinate::from_usize(node_index.index(), self.size))
            .collect::<Vec<_>>();
        for i in 0..cycle_size {
            let weight = self
                .connections
                .edge_weight(self.connections.find_edge(cycle[i], cycle[i + 1]).unwrap())
                .unwrap();
            fields_indexes[i].push(*weight);
            fields_indexes[i + 1].push(*weight);
        }
        fields_indexes[cycle.first().unwrap().index()].push(turn);
        fields_indexes[cycle.last().unwrap().index()].push(turn);
        Cycle::new(fields_coordinates, fields_indexes)
    }
}

mod board_error;
#[cfg(test)]
mod test;
