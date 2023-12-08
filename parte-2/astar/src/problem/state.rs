//! Implementation of a state in the problem

use std::hash::Hash;

use crate::{
    generic::State,
    utils::{BitField, Point},
};

/// State in the search of the problem
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Vehicle {
    /// Non-contagious patients
    pub pacientes_n: u8,
    /// contagious patients
    pub pacientes_c: u8,
    /// Remaining energy
    pub energy: u8,
    /// Current position
    pub pos: Point<usize>,
    /// Picked up patients
    pub visited: BitField,
}

impl State for Vehicle {}

impl Vehicle {
    /// Checks whether the state has finished delivering all patients
    #[must_use]
    pub fn finished(&self) -> bool {
        self.pacientes_n == 0 && self.pacientes_c == 0 && self.visited.all()
    }
}
