//! Definition of a state

use std::hash::Hash;

/// Generic representation of a state in the search
pub trait State: Eq + Hash + Clone {}

// TODO: implement StateCost with cost split between g and h, ordering by h first and using
// buckets+hash map

/// Pair of an state with its associated estimated cost
#[derive(Debug)]
pub struct Cost<T: State> {
    /// State of the search
    pub state: T,
    /// Estimated cost
    pub cost: usize,
}

impl<T: State> PartialOrd for Cost<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: State> Ord for Cost<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl<T: State> PartialEq for Cost<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<T: State> Eq for Cost<T> {}
