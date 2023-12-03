// https://adventofcode.com/2023/day/3

use rangemap::RangeMap;
use std::ops::Range;

use crate::common::get_input;

#[derive(Debug)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SchematicNumber {
    value: i32, // TODO - may need bigint since input can be 140 chars wide (10^140)
    is_part_number: bool,
}

impl SchematicNumber {
    fn new(chars: Vec<char>) -> Self {
        let num_str: String = chars.into_iter().collect();
        let value = num_str.parse().expect("converting string to i32");
        Self {
            value,
            is_part_number: false,
        }
    }
}

pub fn run() {
    let input = get_input("src/day3/input0.txt");

    // Collect all schematic numbers and other symbols
    let mut schematic_numbers = Vec::new();
    let mut symbol_locations = Vec::new();

    let mut row: i32 = 0;
    for line in input {
        let mut schematic_number_in_row = RangeMap::new();
        let mut curr_num_chars = Vec::new();
        let mut curr_num_start_col = -1;
        let mut col: i32 = 0;
        for c in line.chars() {
            if is_number(c) {
                // Append c to curr_num_chars
                if curr_num_chars.len() == 0 {
                    curr_num_start_col = col;
                }
                curr_num_chars.push(c);
            } else {
                if curr_num_chars.len() > 0 {
                    // Done with the input for a number - add it to the map of schematic numbers
                    let col_range = curr_num_start_col..col;
                    schematic_number_in_row.insert(col_range, SchematicNumber::new(curr_num_chars));

                    curr_num_chars = Vec::new();
                    curr_num_start_col = -1;
                }

                if c != '.' {
                    // Found a symbol location
                    symbol_locations.push(Point { row, col });
                }
            }
            col += 1;
        }
        schematic_numbers.push(schematic_number_in_row);
        row += 1;
    }

    println!("Schematic numbers: {:?}", schematic_numbers);
    println!("Symbol locations: {:?}", symbol_locations);
    // TODO now analyze the schematic_numbers and symbol_locations to calculate the answer
}

fn is_number(c: char) -> bool {
    c >= '0' && c <= '9'
}
