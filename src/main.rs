extern crate rust_test;

use std::env;
use std::process;

use rust_test::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {} in {}", cfg.query, cfg.filename);

    if let Err(e) = rust_test::run(cfg) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
