use std::{env, process};

use astar::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = astar::run(&config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
