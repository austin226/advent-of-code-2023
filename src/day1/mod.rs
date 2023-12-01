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
    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").expect("vaild regex");
    let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
    let first = parse_word_digit(matches[0]);
    let last = parse_word_digit(matches[matches.len() - 1]);
    first * 10 + last
}

fn parse_word_digit(word: &str) -> u32 {
    if let Ok(d) = word.parse::<u32>() {
        // word is already a digit
        if (1..=9).contains(&d) {
            return d;
        }
    }
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Invalid word digit: {word}"),
    }
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

    #[test]
    fn parse_word() {
        assert_eq!(parse_word_digit("one"), 1);
    }
}
