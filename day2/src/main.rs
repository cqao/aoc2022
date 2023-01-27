// Problems trying to solve:
//  Part 1: Find your score, based on what you should throw
//  Part 2: Find your score, based on what match result should be

// For reference:
/*
 * A = X = Rock     (1 pts)
 * B = Y = Paper    (2 pts)
 * C = Z = Scissors (3 pts)
 * 
 * W = 6 pts
 * L = 0 pts
 * T = 3 pts
 */

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Opening {}.", args[1]);

    let f = File::open(&args[1]).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut opp_move: Vec<char> = Vec::new();
    let mut my_move: Vec<char> = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");

        opp_move.push(line.chars().next().unwrap());
        my_move.push(line.chars().last().unwrap());
    }

    println!("Opp: {:?}", opp_move);
    println!("Me: {:?}", my_move);

    // Rules:
    //  Scissors > Paper > Rock > Scissors 


}
