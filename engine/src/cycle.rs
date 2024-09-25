use crate::field_coordinate::FieldCoordinate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Debug, PartialEq, Deserialize)]
pub struct Cycle {
    fields_coordinates: Vec<FieldCoordinate>,
    fields_indexes: Vec<Vec<usize>>,
}

impl Cycle {
    pub(super) fn new(
        fields_coordinates: Vec<FieldCoordinate>,
        fields_indexes: Vec<Vec<usize>>,
    ) -> Self {
        Cycle {
            fields_coordinates,
            fields_indexes,
        }
    }

    pub(super) fn get_fields_coordinate(&self) -> &[FieldCoordinate] {
        &self.fields_coordinates
    }

    pub(super) fn get_fields_indexes(&self) -> &[Vec<usize>] {
        &self.fields_indexes
    }

    pub(super) fn len(&self) -> usize {
        self.fields_coordinates.len()
    }

    pub(super) fn shift(&mut self, n: usize) {
        self.fields_coordinates.rotate_left(n);
        self.fields_indexes.rotate_left(n);
    }

    pub(super) fn remove(&mut self, n: usize, edge_weight: usize) -> usize {
        self.fields_indexes[n].retain(|&x| x != edge_weight);
        self.fields_indexes[n][0]
    }

    pub(super) fn get_field_coordinate(&self, n: usize) -> &FieldCoordinate {
        &self.fields_coordinates[n]
    }
}
