use std::collections::HashSet;

use crate::common::get_input;

pub fn run() {
    let input = get_input("src/day4/input0.txt");

    let mut sum = 0;
    for line in input {
        sum += get_scratchcard_value(&parse_line_numbers(line));
    }
    println!("{}", sum);
}

fn parse_line_numbers(line: String) -> (Vec<i32>, Vec<i32>) {
    let numbers: Vec<&str> = line.split(": ").collect();
    let numbers: Vec<&str> = numbers[1].split(" | ").collect();
    let (winning, mine) = (numbers[0], numbers[1]);
    let winning = winning
        .split_whitespace()
        .map(|n| n.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mine = mine
        .split_whitespace()
        .map(|n| n.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    return (winning, mine);
}

fn get_scratchcard_value(line_numbers: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (winning, mine) = line_numbers;
    let w_set: HashSet<i32> = HashSet::from_iter(winning.iter().cloned());

    let mut result = 0;
    for n in mine {
        if w_set.contains(n) {
            if result == 0 {
                result = 1;
            } else {
                result *= 2;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_numbers() {
        assert_eq!(
            parse_line_numbers("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string()),
            (vec![1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14, 1])
        );
    }

    #[test]
    fn test_get_scratchcard_value() {
        for (line, val) in [
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8),
            ("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2),
            ("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2),
            ("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1),
            ("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0),
            ("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0),
        ] {
            assert_eq!(
                get_scratchcard_value(&parse_line_numbers(line.to_string())),
                val
            );
        }
    }
}
