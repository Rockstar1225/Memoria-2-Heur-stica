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
    /// Position of the non-contagious patients
    pub patients_n: Vec<Point<usize>>,
    /// Position of the contagious patients
    pub patients_c: Vec<Point<usize>>,
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
        // Iterates through the tiles in the map
        for (k, tile) in self.grid.iter().enumerate() {
            match tile {
                // If it's a patient, adds its ID and position
                Tile::PacienteC | Tile::PacienteN => {
                    let pos = self.position(k);
                    // targets.patients_id.insert(pos, targets.patients_id.len());
                    match tile {
                        Tile::PacienteN => targets.patients_n.push(pos),
                        Tile::PacienteC => targets.patients_c.push(pos),
                        _ => unreachable!(), // We already know the current tile is a patient
                    }
                }
                // If it's a center/parking, records its position
                Tile::CenterN => targets.center_n = self.position(k),
                Tile::CenterC => targets.center_c = self.position(k),
                Tile::Parking => targets.parking = self.position(k),
                _ => (),
            }
        }
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
}

impl Targets {
    /// Returns an iterator over the positions of all patients
    pub fn all_patients(&self) -> impl Iterator<Item = &Point<usize>> {
        self.patients_n.iter().chain(self.patients_c.iter())
    }

    /// Returns an iterator over the positions of all patients
    pub fn patient_amount(&self) -> usize {
        self.patients_n.len() + self.patients_c.len()
    }

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
        self.all_patients()
            .position(|x| x == pos)
            .expect("There shouldn't be attempts to search for positions that aren't patients")
    }
}
