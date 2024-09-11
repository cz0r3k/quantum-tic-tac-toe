use crate::field_coordinate::FieldCoordinate;
use serde::Serialize;

#[derive(Clone, Serialize, Debug, PartialEq)]
pub struct Cycle {
    fields_coordinates: Vec<FieldCoordinate>,
    fields_indexes: Vec<Vec<usize>>,
}

impl Cycle {
    pub fn new(fields_coordinates: Vec<FieldCoordinate>, fields_indexes: Vec<Vec<usize>>) -> Self {
        Cycle {
            fields_coordinates,
            fields_indexes,
        }
    }

    pub fn get_fields_coordinate(&self) -> &[FieldCoordinate] {
        &self.fields_coordinates
    }

    pub fn get_fields_indexes(&self) -> &[Vec<usize>] {
        &self.fields_indexes
    }

    pub fn len(&self) -> usize {
        self.fields_coordinates.len()
    }
    pub fn shift(&mut self, n: usize) {
        self.fields_coordinates.rotate_left(n);
        self.fields_indexes.rotate_left(n);
    }

    pub fn remove(&mut self, n: usize, edge_weight: usize) -> usize {
        self.fields_indexes[n].retain(|&x| x != edge_weight);
        self.fields_indexes[n][0]
    }

    pub fn get_field_coordinate(&self, n: usize) -> &FieldCoordinate {
        &self.fields_coordinates[n]
    }
}
