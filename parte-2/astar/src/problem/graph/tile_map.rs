//! Implementation of the tile map of the problem

use std::{error::Error, fmt, fs, str::FromStr};

use crate::utils::Point;

// Tile on the map
#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    /// There is nothing special on the tile
    Empty(u8),
    /// The tile can't be traveled through
    Wall,
    /// The tile has a non-contagious patient
    PacienteN,
    /// The tile has a contagious patient
    PacienteC,
    /// The tile has the non-contagious center
    CenterN,
    /// The tile has the contagious center
    CenterC,
    /// The tile has the parking
    Parking,
}

impl Tile {
    // Cost of traveling through the tile
    pub const fn cost(&self) -> u8 {
        if let Self::Empty(e) = self {
            *e
        } else {
            1
        }
    }
}

impl FromStr for Tile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Self::PacienteN),
            "C" => Ok(Self::PacienteC),
            "CN" => Ok(Self::CenterN),
            "CC" => Ok(Self::CenterC),
            "P" => Ok(Self::Parking),
            "X" => Ok(Self::Wall),
            x => Ok(Self::Empty(
                x.parse().map_err(|_| format!("Unknown map tile: `{x}`"))?,
            )),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Empty(e) => write!(f, "{e}"),
            Self::Wall => write!(f, "X"),
            Self::PacienteN => write!(f, "N"),
            Self::PacienteC => write!(f, "C"),
            Self::CenterN => write!(f, "CN"),
            Self::CenterC => write!(f, "CC"),
            Self::Parking => write!(f, "P"),
        }
    }
}

/// Tile map of the problem
pub struct TileMap {
    /// Vector with the tiles
    grid: Vec<Tile>,
    /// Dimensions of the grid
    size: Point<usize>,
}

/// Possible targets information
#[derive(Default, Debug)]
pub struct Targets {
    /// Position of the patients, and estimated cost of
    /// going from them to their center and to the parking
    pub patients: Vec<(Point<usize>, usize)>,
    /// Position of the non-contagious center
    pub center_n: Point<usize>,
    /// Position of the contagious center
    pub center_c: Point<usize>,
    /// Position of the parking
    pub parking: Point<usize>,
}

impl TileMap {
    /// Creates the tile map from a file
    ///
    /// # Parameters
    ///
    /// * `file`: Path of the `.csv` file with the file map to load
    ///
    /// # Errors
    ///
    /// Returns an error if the file couldn't be
    /// opened, or if there was an error parsing it
    pub fn build(file: &str) -> Result<Self, Box<dyn Error>> {
        let file = fs::read_to_string(file)?;
        let height = file.lines().count();
        // Parses the map
        let grid: Vec<_> = file
            // Splits the loaded data by lines
            .lines()
            // Splits each line by ';', and parses each result as a tile
            // Finally, flattens the result as a single iterator
            .flat_map(|line| line.split(';').map(str::parse))
            // Collects the value into a vector
            .collect::<Result<_, _>>()?;
        // Returns the result
        Ok(Self {
            size: Point(height, grid.len() / height),
            grid,
        })
    }

    /// Calculates the position of each target
    #[must_use]
    pub fn get_targets(&self) -> Targets {
        // Initializes the result
        let mut targets = Targets::default();
        let mut patients = vec![];
        // Iterates through the tiles in the map
        for (k, tile) in self.grid.iter().enumerate() {
            // Stores the position of important tiles
            match tile {
                Tile::PacienteC | Tile::PacienteN => patients.push(self.position(k)),
                Tile::CenterN => targets.center_n = self.position(k),
                Tile::CenterC => targets.center_c = self.position(k),
                Tile::Parking => targets.parking = self.position(k),
                _ => (),
            }
        }
        // Calculates the distance of going from each center to the parking
        let dist_n = Self::distance(&targets.center_n, &targets.parking);
        let dist_c = Self::distance(&targets.center_c, &targets.parking);
        let distances = [(&targets.center_n, dist_n), (&targets.center_c, dist_c)];
        // For each patient on the map, calculates the cost of going
        // from its position to its center and to the parking
        targets.patients = patients
            .into_iter()
            .map(|pos| {
                let i = usize::from(*self.tile(&pos) == Tile::PacienteN);
                (pos, Self::distance(&pos, distances[i].0) + distances[i].1)
            })
            .collect();
        targets // Returns the result
    }

    #[must_use]
    /// Calculates the position of a point given its index in the vector
    ///
    /// # Parameters
    ///
    /// * `i`: Index of the tile in the vector
    const fn position(&self, i: usize) -> Point<usize> {
        let x = i / self.size.1;
        Point(x, i - x * self.size.1)
    }

    /// Gets the tile at the given position
    ///
    /// # Parameters
    ///
    /// * `pos`: Position of the tile
    #[must_use]
    pub fn tile(&self, pos: &Point<usize>) -> &Tile {
        &self.grid[pos.1 + pos.0 * self.size.1]
    }

    /// Checks if the given position is a valid position
    ///
    /// # Parameters
    ///
    /// * `pos`: Position to check
    #[must_use]
    pub fn is_traversable(&self, pos: &Point<usize>) -> bool {
        pos.0 < self.size.0 && pos.1 < self.size.1 && *self.tile(pos) != Tile::Wall
    }

    /// Calculates the distance between 2 given positions
    #[must_use]
    pub const fn distance(a: &Point<usize>, b: &Point<usize>) -> usize {
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
    }
}

impl Targets {
    /// Gets the ID of a patient according to their position
    ///
    /// # Parameters
    ///
    /// * `pos`: Position of the patient
    ///
    /// # Panics
    ///
    /// Panics if a position that doesn't correspond to a patient is given
    pub fn id(&self, pos: &Point<usize>) -> usize {
        self.patients
            .iter()
            .position(|(x, _)| x == pos)
            .expect("There shouldn't be attempts to search for positions that aren't patients")
    }
}
