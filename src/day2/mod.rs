// https://adventofcode.com/2023/day/2

use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::common::read_lines;

pub fn run() {
    // Create a path to the desired file
    let path = Path::new("src/day2/input0.txt");
    let mut sum = 0;
    match read_lines(path) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(line) => {
                        // TODO
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

    println!("{}", sum);
}
