mod board_error;
mod lines_result;
#[cfg(test)]
mod test;

use std::collections::HashSet;
use std::iter;
use std::iter::{zip, Peekable};

use crate::board::board_error::BoardError;
use crate::board::lines_result::LinesResult;
use crate::cycle::Cycle;
use crate::field::Field;
use crate::field_coordinate::FieldCoordinate;
use crate::player_symbol::PlayerSymbol;
use crate::DEFAULT_BOARD_SIZE;
use array2d::Array2D;
use error_stack::{Report, Result};
use petgraph::algo::astar;
use petgraph::data::{Element, FromElements};
use petgraph::stable_graph::NodeIndex;
use petgraph::visit::NodeIndexable;
use petgraph::{Graph, Undirected};

#[derive(Debug, Clone)]
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
    pub(super) fn new(size: usize) -> Board {
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

    #[must_use]
    pub fn get_positions(&self) -> Array2D<Field> {
        self.positions.clone()
    }

    pub(super) fn mark(
        &mut self,
        fields_coordinates: &[FieldCoordinate],
        player_symbol: PlayerSymbol,
        turn: usize,
    ) -> Result<Option<Cycle>, BoardError> {
        let mut hash_set = HashSet::<FieldCoordinate>::new();
        // Check for errors
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
                                .attach_printable("Field is already entangled"));
                        }
                        Ok(field.clone())
                    }
                    None => Err(Report::new(BoardError::new(field_coordinate))
                        .attach_printable("Out of band coordinate")),
                }
            })
            .try_collect::<Vec<Field>>()?;

        // zip fields_coordinates with fields
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
                .expect("Coordinates should be valid");
        });

        let nodes = fields_coordinates
            .iter()
            .map(|&field_coordinate| {
                self.connections
                    .from_index(FieldCoordinate::into_usize(field_coordinate, self.size))
            })
            .collect::<Vec<_>>();

        // Check for cycle
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

    pub(super) fn collapse(
        &mut self,
        field_coordinate: FieldCoordinate,
        index: usize,
    ) -> Result<(), BoardError> {
        // Check for errors
        let mut last_cycle = match &self.last_cycle {
            Some(cycle) => cycle.clone(),
            None => {
                return Err(Report::new(BoardError::new(field_coordinate))
                    .attach_printable("No cycle found"))
            }
        };
        let Some(n) = last_cycle
            .get_fields_coordinate()
            .iter()
            .position(|&coordinate| coordinate == field_coordinate)
        else {
            return Err(Report::new(BoardError::new(field_coordinate))
                .attach_printable("No coordinate found in cycle"));
        };
        if !last_cycle.get_fields_indexes()[n].contains(&index) {
            return Err(Report::new(BoardError::new(field_coordinate))
                .attach_printable("No index found in coordinate"));
        }

        //Collapse cycle
        last_cycle.shift(n);
        let mut last_edge_weight = index;
        let mut last_field_coordinate = field_coordinate;
        let cycle_len = last_cycle.len();

        for i in 0..cycle_len {
            last_edge_weight = last_cycle.remove((i + 1) % cycle_len, last_edge_weight);
            let field_coordinate = last_cycle.get_field_coordinate((i + 1) % cycle_len);
            let player_symbol =
                self.get_player_symbol_from_entangled(field_coordinate, last_edge_weight)?;
            self.set_collapse(field_coordinate, player_symbol);
            self.remove_edge(field_coordinate, &last_field_coordinate);
            last_field_coordinate = *field_coordinate;
        }

        //Collapse outside cycle
        let mut nodes_indexes = last_cycle
            .get_fields_coordinate()
            .iter()
            .map(|coordinate| self.get_node(coordinate))
            .collect::<Vec<_>>();

        while let Some(node) = nodes_indexes.pop() {
            let neighbors = self.connections.neighbors(node);
            let node_coordinate = FieldCoordinate::from_usize(node.index(), self.size);
            let mut to_collapse = Vec::new();
            for neighbor in neighbors {
                nodes_indexes.push(neighbor);
                let neighbor_coordinate = FieldCoordinate::from_usize(neighbor.index(), self.size);
                if let Some(edge) = self.connections.find_edge(node, neighbor) {
                    let player_symbol = self.get_player_symbol_from_entangled(
                        &neighbor_coordinate,
                        *self
                            .connections
                            .edge_weight(edge)
                            .expect("Edge should exist"),
                    )?;
                    to_collapse.push((neighbor_coordinate, player_symbol));
                }
            }
            for (neighbor_coordinate, player_symbol) in to_collapse {
                self.set_collapse(&neighbor_coordinate, player_symbol);
                self.remove_edge(&node_coordinate, &neighbor_coordinate);
            }
        }
        Ok(())
    }

    fn get_player_symbol_from_entangled(
        &self,
        field_coordinate: &FieldCoordinate,
        index: usize,
    ) -> Result<PlayerSymbol, BoardError> {
        match self
            .positions
            .get(field_coordinate.y, field_coordinate.x)
            .expect("Coordinate should be valid")
        {
            Field::Entangled(symbols) => Ok(symbols[index].expect("Index should be valid")),
            Field::Collapsed(_) => Err(Report::new(BoardError::new(*field_coordinate))
                .attach_printable("Field is collapsed")),
        }
    }

    fn set_collapse(&mut self, field_coordinate: &FieldCoordinate, player_symbol: PlayerSymbol) {
        self.positions
            .set(
                field_coordinate.y,
                field_coordinate.x,
                Field::Collapsed(player_symbol),
            )
            .expect("Coordinate should be valid");
    }

    fn remove_edge(
        &mut self,
        first_coordinate: &FieldCoordinate,
        second_coordinate: &FieldCoordinate,
    ) {
        if let Some(edge) = self.connections.find_edge(
            self.get_node(first_coordinate),
            self.get_node(second_coordinate),
        ) {
            self.connections.remove_edge(edge);
        };
    }

    fn get_node(&self, field_coordinate: &FieldCoordinate) -> NodeIndex {
        self.connections
            .from_index(FieldCoordinate::into_usize(*field_coordinate, self.size))
    }
    fn map_cycle(&self, cycle: Option<(usize, Vec<NodeIndex>)>, turn: usize) -> Cycle {
        let cycle = cycle.expect("Cycle should exist");
        let cycle_size = cycle.0;
        let cycle = cycle.1;
        let mut fields_indexes = vec![Vec::<usize>::new(); cycle_size + 1];
        let fields_coordinates = cycle
            .iter()
            .map(|node_index| FieldCoordinate::from_usize(node_index.index(), self.size))
            .collect::<Vec<_>>();
        for i in 0..cycle_size {
            let weight = self
                .connections
                .edge_weight(
                    self.connections
                        .find_edge(cycle[i], cycle[i + 1])
                        .expect("Edge should exist"),
                )
                .expect("Edge should exist");
            fields_indexes[i].push(*weight);
            fields_indexes[i + 1].push(*weight);
        }
        fields_indexes[0].push(turn);
        fields_indexes[cycle_size].push(turn);
        Cycle::new(fields_coordinates, fields_indexes)
    }

    pub(super) fn check_all_lines(&self) -> LinesResult {
        let mut lines_result = LinesResult::new();
        self.check_rows()
            .iter()
            .for_each(|&player_symbol| lines_result.increase(player_symbol));
        self.check_columns()
            .iter()
            .for_each(|&player_symbol| lines_result.increase(player_symbol));
        self.check_diagonals()
            .iter()
            .for_each(|&player_symbol| lines_result.increase(player_symbol));
        lines_result
    }

    fn check_rows(&self) -> Vec<PlayerSymbol> {
        (0..self.positions.row_len())
            .filter_map(|row| self.check_row(row))
            .collect::<Vec<PlayerSymbol>>()
    }

    fn check_columns(&self) -> Vec<PlayerSymbol> {
        (0..self.positions.column_len())
            .filter_map(|column| self.check_column(column))
            .collect::<Vec<PlayerSymbol>>()
    }

    fn check_diagonals(&self) -> Vec<PlayerSymbol> {
        let mut symbols = Vec::new();
        if let Some(symbol) = self.check_first_diagonal() {
            symbols.push(symbol);
        }
        if let Some(symbol) = self.check_second_diagonal() {
            symbols.push(symbol);
        }
        symbols
    }

    fn check_row(&self, row: usize) -> Option<PlayerSymbol> {
        let iter = self
            .positions
            .row_iter(row)
            .expect("Row number should be valid")
            .peekable();
        Board::check_line(iter)
    }

    fn check_column(&self, column: usize) -> Option<PlayerSymbol> {
        let iter = self
            .positions
            .column_iter(column)
            .expect("Column number should be valid")
            .peekable();
        Board::check_line(iter)
    }

    fn check_first_diagonal(&self) -> Option<PlayerSymbol> {
        Board::check_line((0..self.size).map(|i| &self.positions[(i, i)]).peekable())
    }

    fn check_second_diagonal(&self) -> Option<PlayerSymbol> {
        Board::check_line(
            (0..self.size)
                .map(|i| &self.positions[(i, self.size - i - 1)])
                .peekable(),
        )
    }

    fn check_line<'a, I>(mut line: Peekable<I>) -> Option<PlayerSymbol>
    where
        I: Iterator<Item = &'a Field>,
    {
        let first = *line.peek().expect("Should bo item to pick");
        if matches!(first, Field::Entangled(_)) {
            return None;
        } else if line.all(|field| field == first) {
            if let Field::Collapsed(symbol) = first {
                return Some(*symbol);
            }
        }
        None
    }
}
