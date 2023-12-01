// https://adventofcode.com/2023/day/1

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
    let regex = Regex::new(r"(?:(\d).*(\d).*|(\d).*)").expect("vaild regex");
    for caps in regex.captures_iter(line) {
        if caps.len() != 4 {
            panic!("line '{line}' has {} captures", caps.len());
        }

        let (first, last) = if let (Some(first), Some(last)) = (caps.get(1), caps.get(2)) {
            (first, last)
        } else if let Some(only) = caps.get(3) {
            (only, only)
        } else {
            panic!("Invalid line - {line}");
        };

        let tens = first.as_str().parse::<u32>().unwrap() * 10;
        let ones = last.as_str().parse::<u32>().unwrap();
        return tens + ones;
    }
    panic!("No captures in line {line}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_number() {
        for (line, exp_res) in [
            ("1abc2", 12),
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
        ] {
            assert_eq!(exp_res, secret_number(&line.to_string()), "line={line}");
        }
    }
}
