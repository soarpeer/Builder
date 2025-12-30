use std::env;
use std::process::exit;
use std::collections::HashMap;
use crate::parser::{parse_buildfile, Target};
use crate::builder::build_target;
use crate::cleaner::clean;
use crate::runner::run;

mod parser;
mod builder;
mod cleaner;
mod runner;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: build <build|clean|run>");
        exit(1);
    }

    let targets = parse_buildfile("buildfile.txt").expect("Failed to parse buildfile");

    match args[1].as_str() {
        "build" => {
            build_target("build", &targets); // Start building from the 'build' target
        }
        "clean" => {
            clean(&targets); // Clean all the generated files
        }
        "run" => {
            run(&targets); // Run the program (if the 'run' target exists)
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            exit(1);
        }
    }
}
