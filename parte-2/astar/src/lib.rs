use std::error::Error;

pub mod generic;
pub mod problem;
pub mod utils;

/// Program configuration struct
pub struct Config {
    /// Input file name
    pub input_file: String,
    /// Heuristic to use ("1" | "2")
    pub heuristic: String,
}

impl Config {
    /// Creates a new instance of the program configuration
    ///
    /// # Parameters
    ///
    /// * `args`: Iterator of command-line arguments
    ///
    /// # Errors
    ///
    /// Returns an error if the arguments couldn't be parsed correctly
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next(); // Ignore program name argument
        let Some(input_file) = args.next() else {
            return Err("Missing input file");
        };
        let Some(heuristic) = args.next() else {
            return Err("Missing heuristic");
        };
        Ok(Self {
            input_file,
            heuristic,
        })
    }
}

use generic::AStar;
use problem::Graph;

/// Executes the program
///
/// # Parameters
///
/// * `config`: Configuration of the program
///
/// # Errors
///
/// Returns an error if there was an error running the program
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let map = Graph::build(&config.input_file)?;
    let (result, stats) = if config.heuristic == "1" {
        map.a_star(map.start_state(), |state| map.h1(state))
    } else {
        map.a_star(map.start_state(), |state| map.h2(state))
    };
    println!("Elapsed time: {:.4?}", stats.time);
    println!("Expanded nodes: {}", stats.expanded);
    result.as_ref().map_or_else(
        || println!("No path found"),
        |res| {
            println!("Path cost: {}", res.cost);
            println!("Path length: {}", res.path.len());
            println!("Path:");
            for state in &res.path {
                println!("{state:?}");
            }
        },
    );
    map.dump(
        &format!(
            "{}-{}",
            &config.input_file[..config.input_file.len() - 4],
            config.heuristic
        ),
        result.as_ref(),
        &stats,
    )?;
    Ok(())
}
