
// Each ruck has 2 compartments
// Each item is meant to go into only one of the compartments
// Items identified by lowercase or uppercase letter
// Each item has a priotiry assigned: 'a' through 'z' has priority values 1 - 26
// Each item has a priotiry assigned: 'A' through 'Z' has priority values 27 - 52
// Each ruck compartment shares an item

// GOAL: Add the priority values for each shared component


use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Opening {}.", args[1]);

    let f = File::open(&args[1]).expect("Unable to open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Unable to read line");

        println!("{}", line);
    }
}