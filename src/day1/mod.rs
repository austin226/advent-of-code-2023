// https://adventofcode.com/2023/day/1

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn run() {
    // Create a path to the desired file
    let path = Path::new("src/day1/input.txt");

    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}
