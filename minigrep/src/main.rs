use std::env;
use std::process;

mod lib;
use lib::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Failed to parse config: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        println!("Failed to read file: {}", err);
        process::exit(1);
    }
}
