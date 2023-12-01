// https://adventofcode.com/2023/day/1

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn run() {
    // Create a path to the desired file
    let path = Path::new("src/day1/input.txt");

    match read_lines(path) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(line) => {
                        println!("{}", line);
                    }
                    Err(err) => {
                        panic!("Cannot read line in {} - {}", path.display(), err);
                    }
                }
            }
        }
        Err(err) => {
            panic!("Cannot read file {} - {}", path.display(), err);
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

/// Add the first and last digit characters in the string together.
fn line_sum(line: String) -> u32 {
    // TODO
    0
}
