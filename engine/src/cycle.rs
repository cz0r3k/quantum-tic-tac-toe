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

    pub fn get_fields_coordinate(&self) -> &Vec<FieldCoordinate> {
        &self.fields_coordinates
    }

    pub fn get_fields_indexes(&self) -> &Vec<Vec<usize>> {
        &self.fields_indexes
    }

    pub fn len(&self) -> usize {
        self.fields_coordinates.len()
    }
}
