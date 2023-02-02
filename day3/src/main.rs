// Solve:
//  Part 1: Add the priority values for each shared component
//  Part 2: Add the priority values for each component shared across 3 lines  

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::{Seek, SeekFrom};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Opening {}.", args[1]);

    let mut f = File::open(&args[1]).expect("Unable to open file");

    println!("Total Priority Part 1: {}", part_one(&f));

    f.seek(SeekFrom::Start(0)).expect("Seek reset failed");
    println!("Total Priority Part 2: {}", part_two(&f));
}

fn part_one(f: &File) -> i32 {

    let f = BufReader::new(f);
    let mut sum = 0;
    let priority: HashMap<char, i32> = populate_map();
    let mut exists: bool = false;

    for line in f.lines() {
        let line = line.expect("Unable to read line");

        let (one, two) = line.split_at(line.chars().count() / 2);

        for c in one.chars() {
            if two.contains(c) {

                if !exists {
                    sum = sum + priority.get(&c).unwrap();
                    exists = true;
                }
                
            }
        }

        exists = false;
    }
    
    return sum;
}

fn part_two(f: &File) -> i32 {

    let f = BufReader::new(f);
    let mut v: Vec<String> = Vec::new();
    let mut sum = 0;
    
    let priority: HashMap<char, i32> = populate_map();
    let mut exists: bool = false;
    
    for line in f.lines() {
        let line = line.expect("Unable to read line");

        v.push(line);
    }

    for i in (0..v.len()).step_by(3) {
        let s1 = &v[i];
        let s2 = &v[i+1];
        let s3 = &v[i+2];

        for c in s1.chars() {
            if s2.contains(c) && s3.contains(c) {

                if !exists {
                    sum = sum + priority.get(&c).unwrap();
                    exists = true;
                }
                
            }
        }

        exists = false;
    }

    return sum;
}

fn populate_map() -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();

    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);
    map.insert('d', 4);
    map.insert('e', 5);
    map.insert('f', 6);
    map.insert('g', 7);
    map.insert('h', 8);
    map.insert('i', 9);
    map.insert('j', 10);
    map.insert('k', 11);
    map.insert('l', 12);
    map.insert('m', 13);
    map.insert('n', 14);
    map.insert('o', 15);
    map.insert('p', 16);
    map.insert('q', 17);
    map.insert('r', 18);
    map.insert('s', 19);
    map.insert('t', 20);
    map.insert('u', 21);
    map.insert('v', 22);
    map.insert('w', 23);
    map.insert('x', 24);
    map.insert('y', 25);
    map.insert('z', 26);
    
    map.insert('A', 27);
    map.insert('B', 28);
    map.insert('C', 29);
    map.insert('D', 30);
    map.insert('E', 31);
    map.insert('F', 32);
    map.insert('G', 33);
    map.insert('H', 34);
    map.insert('I', 35);
    map.insert('J', 36);
    map.insert('K', 37);
    map.insert('L', 38);
    map.insert('M', 39);
    map.insert('N', 40);
    map.insert('O', 41);
    map.insert('P', 42);
    map.insert('Q', 43);
    map.insert('R', 44);
    map.insert('S', 45);
    map.insert('T', 46);
    map.insert('U', 47);
    map.insert('V', 48);
    map.insert('W', 49);
    map.insert('X', 50);
    map.insert('Y', 51);
    map.insert('Z', 52);

    return map;
}