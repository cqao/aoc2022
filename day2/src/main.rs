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

    let mut total_part_one = 0;
    let mut total_part_two = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");

        let opp_move: char = line.chars().next().unwrap();
        let my_move:  char = line.chars().last().unwrap();
    
        total_part_one = total_part_one + part_one(&opp_move, &my_move);
        total_part_two = total_part_two + part_two(&opp_move, &my_move);
    }

    println!("Total score for Part 1: {}", total_part_one);
    println!("Total score for Part 2: {}", total_part_two);

}

pub fn part_one(om: &char, mm: &char) -> i32 {
    let mut t = 0;

    if *mm == 'X' {
        t = t + 1;
    }

    else if *mm == 'Y' {
        t = t + 2;
    }

    else if *mm == 'Z' {
        t = t + 3;
    }

    // Paper beats Rock

    if *om == 'A' && *mm == 'Y' {
        t = t + 6;
    }

    // Scissors beat Paper
    if *om == 'B' && *mm == 'Z' {
        t = t + 6;
    }

    // Rock beats Scissors
    if *om == 'C' && *mm == 'X' {
        t = t + 6;
    }

    // Handle ties

    if (*om == 'A' && *mm == 'X') ||
       (*om == 'B' && *mm == 'Y') || 
       (*om == 'C' && *mm == 'Z') {
        t = t + 3;
    }

    //println!("Line {}: {}", om, t);
    return t;
}

pub fn part_two(om: &char, mm: &char) -> i32 {

    let mut t = 0;

    // Ties

    if *om == 'A' && *mm == 'Y' {
        t = t + 4;
    }

    else if *om == 'B' && *mm == 'Y' {
        t = t + 5;
    }

    else if *om == 'C' && *mm == 'Y'{
        t = t + 6;
    }

    // Wins
    if *om == 'A' && *mm == 'Z'{
        // Paper
        t = t + 8;
    }

    else if *om == 'B' && *mm == 'Z'{
        // Scissors
        t = t + 9;
    }

    else if *om == 'C' && *mm == 'Z'{
        // Rock
        t = t + 7;
    }

    // Loses

    if *om == 'A' && *mm == 'X'{
        // Scissors
        t = t + 3;
    }

    else if *om == 'B' && *mm == 'X'{
        // Rock
        t = t + 1;
    }

    else if *om == 'C' && *mm == 'X'{
        // Paper
        t = t + 2;
    }

    return t;
}