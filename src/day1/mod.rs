// https://adventofcode.com/2023/day/1

use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const DIGITS: [(&str, u32); 19] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

struct DigitMatch {
    digit: u32,
    index: usize,
}

pub fn run() {
    // Create a path to the desired file
    let path = Path::new("src/day1/input2.txt");
    let mut sum = 0;
    match read_lines(path) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(line) => {
                        sum += secret_number(&line);
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

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}

/// Concatenate the first and last digit characters in the string together.
fn secret_number(line: &String) -> u32 {
    // contruct a map of digit matches to positions
    let mut matches = Vec::new();
    for (d_str, d_val) in DIGITS {
        for (index, _) in line.match_indices(d_str) {
            matches.push(DigitMatch {
                digit: d_val,
                index,
            });
        }
    }

    // Sort matches by index
    matches.sort_by(|a, b| a.index.cmp(&b.index));
    let matches = matches;
    let first = &matches[0];
    let last = &matches[matches.len() - 1];
    return first.digit * 10 + last.digit;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_number() {
        for (line, exp_res) in [
            ("eighthree", 83),
            ("sevenine", 79),
            ("1abc2two", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
            ("12aaaaaa", 12),
            ("1aaaa2aaa", 12),
            ("aaa1aa2aaa", 12),
            ("1aa2", 12),
            ("aa1aa2", 12),
            ("aa12aa", 12),
            ("12345", 15),
            ("one2three", 13),
            ("1two", 12),
            ("1four7", 17),
            ("asixa", 66),
        ] {
            assert_eq!(exp_res, secret_number(&line.to_string()), "line={line}");
        }
    }
}
