//! Generic implementation of the A* algorithm

mod state;
use state::Cost;
pub use state::State;

mod astar;
pub use astar::{AStar, SearchResult, Stats};
