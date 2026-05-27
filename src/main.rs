mod analyzer;
mod harness;
mod mutator;
mod fuzzer;

use std::env;
use analyzer::analyze_binary;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <binary_path>", args[0]);
        return;
    }

    let path = Path::new(&args[1]);
    match analyze_binary(path) {
        Some(report) => {
            println!("Path: {:?}", report.path);
            println!("Score: {}", report.score);
            println!("Danger funcs: {:?}", report.danger_funcs);
            println!("Input funcs: {:?}", report.input_funcs);
        }
        None => println!("No interesting functions found"),
    }
}