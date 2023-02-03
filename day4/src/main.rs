// Problem to solve:
//  Part 1: Find how many pairs when one range fully contains the other
//  Part 2: Find all overlapping pairs

// TODO: seperate out part 1 and part 2 into seperate functions

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Opening {}.", args[1]);

    let f = File::open(&args[1]).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut vec: Vec<String> = Vec::new();
    let mut pair: i32 = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");

        let split = line.split(',');

        for s in split {
            let split2 = s.split('-');
            for s2 in split2 {
                vec.push(s2.to_string());
            }
        }
    }

    for i in (0..vec.len()).step_by(4) {
        let n: i32 = vec[i].parse().unwrap();
        let n1: i32 = vec[i+1].parse().unwrap();
        let n2: i32 = vec[i+2].parse().unwrap();
        let n3: i32 = vec[i+3].parse().unwrap();

        if n1 == n2 {
            // Single Match (5-7,7-9)
            println!("Match Single: {}-{},{}-{}", n, n1, n2, n3);
            pair = pair + 1;
        }

        else if n <= n2 && n1 >= n3 {
            // Left contains right (1-5,2-4) or (2-5,2-4)
            println!("Match LcR: {}-{},{}-{}", n, n1, n2, n3);
            pair = pair + 1;
        }

        else if n2 <= n && n3 >= n1 {
            // Right contains left (2-4,1-5) or (6-6,4-6)
            println!("Match RcL: {}-{},{}-{}", n, n1, n2, n3);
            pair = pair + 1;
        }

        else if n <= n2 && n2 < n1 && n1 <= n3 {
            // partial overlap (1-5,3-6) or (1-5,1-6)
            println!("Match Partial RL: {}-{},{}-{}", n, n1, n2, n3);
            pair = pair + 1;
        }

        else if n2 <= n && n <= n3 && n1 > n3 {
            // partial overlap (3-7,1-6) or (1-7,3-)
            println!("Match Partial LR: {}-{},{}-{}", n, n1, n2, n3);
            pair = pair + 1;
        }
    }

    println!("Total pairs contained: {}", pair);
}
