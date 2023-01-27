// Problems tyring to solve:
//   Part 1: Need to find the elf with the most calories and report how many calories they have.
//   Part 2: Add the top 3 calorie carrying elves

// Step 1: Read file, pause on newline
// Step 2: Add numbers to get an elf's total
// Step 3: Put totals into a vector
// Step 4: Sort and find answers

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Opening {}.", args[1]);

    let f = File::open(&args[1]).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut elf_totals: Vec<i32> = Vec::new();
    let mut n: i32;
    let mut total: i32 = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");

        
        if line.eq("") {
            elf_totals.push(total);
            total = 0;
        }
        else {
            n = line.parse().unwrap();
            total = total + n;
        }
    }

    //println!("Total: {:?}", elf_totals);

    elf_totals.sort();

    let part2_total: i32 = elf_totals[elf_totals.len()-1] + elf_totals[elf_totals.len()-2] + elf_totals[elf_totals.len()-3];
    
    println!("Part 1: Largest Total Calories: {}", elf_totals[elf_totals.len() - 1]);
    println!("Part 2: Top 3 Total Calories: {}", part2_total);
    



}
