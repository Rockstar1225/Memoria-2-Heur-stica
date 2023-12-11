use std::{env, process};

use astar::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("\x1B[31mError Parsing Arguments:\x1B[0m {err}");
        process::exit(1);
    });
    if let Err(e) = astar::run(&config) {
        println!("\x1B[31mApplication Error:\x1B[0m {e}");
        process::exit(1);
    }
}
