// https://adventofcode.com/2023/day/3

use std::ops::Range;

use crate::common::get_input;

struct Point {
    row: i32,
    col: i32,
}

struct SchematicNumber {
    start: Point,
    end: Point,
    value: i32, // TODO - may need bigint since input can be 140 chars wide (10^140)
    is_part_number: bool,
}

impl SchematicNumber {
    fn new(row: i32, col_range: Range<i32>, chars: Vec<char>) -> Self {
        Self {
            // TODO
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
                    // Done with the input for a number - add it to the list of schematic numbmers
                    schematic_numbers.push(SchematicNumber::new(
                        row,
                        curr_num_start_col..col,
                        curr_num_chars,
                    ));
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
        row += 1;
    }

    // TODO now analyze the schematic_numbers and symbol_locations to calculate the answer
}

fn is_number(c: char) -> bool {
    // TODO
    false
}

fn is_symbol(c: char) -> bool {
    // TODO
    false
}
