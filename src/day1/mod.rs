// https://adventofcode.com/2023/day/1

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn run() {
    println!("Day 1");

    // Create a path to the desired file
    let path = Path::new("src/day1/input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}
