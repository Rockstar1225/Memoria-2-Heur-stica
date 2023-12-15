//! Implementation of the graph of the problem with the global (constant) data

use std::{error::Error, fmt::Write, fs, io};

use super::Vehicle;
use crate::{
    generic::{AStar, SearchResult, Stats},
    utils::{BitField, Point},
};

mod tile_map;
use tile_map::{Targets, Tile, TileMap};

/// Representation of the problem search graph, with the global constant state
pub struct Graph {
    /// Maximum amount of patients picked up at once
    plazas: u8,
    /// Maximum amount of contagious patients picked up at once
    plazas_c: u8,
    /// Starting energy at the parking
    start_energy: u8,
    /// Tile map of the problem
    grid: TileMap,
    /// Location of important destinations
    targets: Targets,
}

impl Graph {
    /// Creates a new instance of the Graph
    ///
    /// # Parameters
    ///
    /// * `file`: Path of the `.csv` file with the file map to load
    ///
    /// # Errors
    ///
    /// Returns an error if the tile map couldn't be loaded
    pub fn build(file: &str) -> Result<Self, Box<dyn Error>> {
        let grid = TileMap::build(file)?;
        let targets = grid.get_targets();
        Ok(Self {
            plazas: 10,
            plazas_c: 2,
            start_energy: 50,
            grid,
            targets,
        })
    }

    /// Dumps the result of a search to a pair of files (result and statistics)
    ///
    /// # Parameters
    ///
    /// * `filename`: Input filename from which the path/name of the output files will be extracted
    /// * `path`: Optimal path to write
    /// * `stats`: Statistics of the search
    ///
    /// # Errors
    ///
    /// Returns an error if either of the files couldn't be written
    pub fn dump(
        &self,
        filename: &str,
        path: Option<&SearchResult<Vehicle>>,
        stats: &Stats,
    ) -> io::Result<()> {
        let mut data = String::new();
        let (cost, length) = path.map_or(("-".to_string(), "-".to_string()), |res| {
            data.reserve(res.path.len() * 9);
            for state in &res.path {
                writeln!(
                    data,
                    "({},{}):{}:{}",
                    state.pos.0 + 1,
                    state.pos.1 + 1,
                    self.grid.tile(&state.pos),
                    state.energy
                )
                .expect("Appending to a `String` should never error");
            }
            (res.cost.to_string(), res.path.len().to_string())
        });
        fs::write(format!("{filename}.output"), data)?;
        fs::write(
            format!("{filename}.stat"),
            format!(
                "Tiempo total: {:.4?}\nCoste total: {}\nLongitud del plan: {}\nNodos Expandidos: {}",
                stats.time,
                cost,
                length,
                stats.expanded
            ),
        )?;
        Ok(())
    }

    /// Checks whether an state can pick up a non-contagious patient
    ///
    /// # Parameters
    ///
    /// * `state`: State to check
    const fn can_pickup_n(&self, state: &Vehicle) -> bool {
        state.pacientes_c == 0 && state.pacientes_n < self.plazas
    }

    /// Checks whether an state can pick up a contagious patient
    ///
    /// # Parameters
    ///
    /// * `state`: State to check
    const fn can_pickup_c(&self, state: &Vehicle) -> bool {
        state.pacientes_c < self.plazas_c && state.pacientes_n <= self.plazas - self.plazas_c
    }

    /// Creates the starting state of the search
    ///
    /// # Parameters
    ///
    /// * `state`: State to check
    #[must_use]
    pub fn start_state(&self) -> Vehicle {
        Vehicle {
            pacientes_n: 0,
            pacientes_c: 0,
            energy: self.start_energy,
            pos: self.targets.parking,
            visited: BitField::new(self.targets.patients.len()),
        }
    }

    /// Creates a new state that is the result of moving the
    /// position in the given state in the given direction
    ///
    /// # Parameters
    ///
    /// * `state`: State to move
    /// * `offset`: Direction in which to move the state
    fn move_state(&self, state: &Vehicle, offset: Point<i8>) -> Option<(Vehicle, usize)> {
        // Clone the state to not modify the original
        let mut state = state.clone();
        // Move in the chosen direction. If the tile isn't traversable, stop the expansion
        state.pos = (state.pos + offset)?;
        self.grid.is_traversable(&state.pos).then_some(())?;
        // Calculate the cost
        let tile = self.grid.tile(&state.pos);
        let cost = tile.cost();
        // If there isn't enough energy, stop the expansion
        state.energy = state.energy.checked_sub(cost)?;
        // Pickup/drop patients
        match tile {
            Tile::PacienteN | Tile::PacienteC => {
                // There is a patient in the current tile. Pick
                // it up if possible and wasn't already picked up
                let i = self.targets.id(&state.pos);
                if !state.visited.get(i) {
                    if *tile == Tile::PacienteN && self.can_pickup_n(&state) {
                        state.pacientes_n += 1;
                        state.visited.set(i);
                    } else if *tile == Tile::PacienteC && self.can_pickup_c(&state) {
                        state.pacientes_c += 1;
                        state.visited.set(i);
                    }
                }
            }
            // If it's a non-contagious center, drop all non-contagious
            // patients if there aren't any contagious patients
            Tile::CenterN if state.pacientes_c == 0 => state.pacientes_n = 0,
            // If it's a contagious center, drop all contagious patients
            Tile::CenterC => state.pacientes_c = 0,
            // If it's the parking, reset the amount of energy
            Tile::Parking => state.energy = self.start_energy,
            _ => (), // Otherwise, do nothing
        }
        Some((state, cost.into()))
    }

    /// Estimates the cost of delivering the indicated patient types and going back to the parking
    ///
    /// # Parameters
    ///
    /// * `pos`: Current position
    /// * `pacientes_c`: Whether to deliver contagious patients
    /// * `pacientes_n`: Whether to deliver non-contagious patients
    #[must_use]
    const fn finish_cost(
        &self,
        mut pos: Point<usize>,
        pacientes_c: bool,
        pacientes_n: bool,
    ) -> usize {
        let mut distance = 0;
        // If there are contagious patients, goes to the contagious center to drop them
        if pacientes_c {
            distance += TileMap::distance(&pos, &self.targets.center_c);
            pos = self.targets.center_c;
        }
        // If there are non-contagious patients, goes to the non-contagious center to drop them
        if pacientes_n {
            distance += TileMap::distance(&pos, &self.targets.center_n);
            pos = self.targets.center_n;
        }
        // Adds the distance to return to the parking
        distance + TileMap::distance(&pos, &self.targets.parking)
    }

    fn max_cost(
        &self,
        state: &Vehicle,
        get_cost: impl Fn(&Vehicle, &(Point<usize>, usize)) -> usize,
    ) -> usize {
        self.targets
            .patients
            .iter()
            .enumerate()
            // Filters the visited positions
            .filter(|(i, _)| !state.visited.get(*i))
            // Calculates the distance to each patient and adds the cost of finishing from that
            // patient
            .map(|(_, x)| get_cost(state, x))
            // Get the maximum distance value
            .max()
            .unwrap_or_else(|| {
                self.finish_cost(state.pos, state.pacientes_c != 0, state.pacientes_n != 0)
            })
    }

    // First heuristic
    #[must_use]
    pub fn h1(&self, state: &Vehicle) -> usize {
        self.max_cost(state, |_, (_, cost)| *cost)
    }

    // Second heuristic
    #[must_use]
    pub fn h2(&self, state: &Vehicle) -> usize {
        self.max_cost(state, |state, (pos, cost)| {
            TileMap::distance(&state.pos, pos) + cost
        })
    }
}

impl AStar for Graph {
    type State = Vehicle;
    fn next(&self, state: &Self::State) -> Vec<(Self::State, usize)> {
        [Point(0, 1), Point(0, -1), Point(1, 0), Point(-1, 0)]
            .into_iter()
            .filter_map(|offset| self.move_state(state, offset))
            .collect()
    }

    fn is_goal(&self, state: &Self::State) -> bool {
        state.finished() && state.pos == self.targets.parking
    }
}
