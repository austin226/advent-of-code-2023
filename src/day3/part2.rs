// https://adventofcode.com/2023/day/3

use std::ops::RangeInclusive;

use rangemap::RangeInclusiveMap;

use crate::common::get_input;

#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn neighbors(&self, n_rows: usize, n_cols: usize) -> Vec<Point> {
        let mut neighbors = Vec::new();

        for (dx, dy) in [
            (-1, 1),
            (0, 1),
            (1, 1),
            (-1, 0),
            (1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ] {
            let row = self.row as i32 + dy;
            let col = self.col as i32 + dx;
            if (0..n_rows as i32).contains(&row) && (0..n_cols as i32).contains(&col) {
                neighbors.push(Point::new(row as usize, col as usize));
            }
        }

        neighbors
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SchematicNumber {
    value: i32,
    row: usize,
    range: RangeInclusive<usize>,
}

pub fn run() {
    let input = get_input("src/day3/input1.txt");

    // Handle 0 case
    let n_rows = input.len();
    if n_rows == 0 {
        println!("0");
        return;
    }
    let n_cols = input[0].len();
    if n_cols == 0 {
        println!("0");
        return;
    }

    // Collect all schematic numbers and other symbols
    let mut schematic_numbers = Vec::new();
    let mut star_locations = Vec::new();

    let mut row: usize = 0;
    for line in input {
        let mut schematic_numbers_in_row = RangeInclusiveMap::new();
        let mut curr_num_chars = Vec::new();
        let mut curr_num_start_col: Option<usize> = None;
        let mut col: usize = 0;
        for c in line.chars() {
            if is_number(c) {
                // Append c to curr_num_chars
                if curr_num_chars.len() == 0 {
                    curr_num_start_col = Some(col);
                }
                curr_num_chars.push(c);
            } else {
                if let Some(start_col) = curr_num_start_col {
                    // Done with the input for a number - add it to the map of schematic numbers
                    let range = start_col..=(col - 1);
                    schematic_numbers_in_row.insert(
                        range.clone(),
                        SchematicNumber {
                            value: parse_num(curr_num_chars),
                            row,
                            range,
                        },
                    );

                    curr_num_chars = Vec::new();
                    curr_num_start_col = None;
                }

                if c == '*' {
                    // Found a star location
                    star_locations.push(Point { row, col });
                }
            }
            col += 1;
        }

        // Handle number at end of line
        if let Some(start_col) = curr_num_start_col {
            // Done with the input for a number - add it to the map of schematic numbers
            let range = start_col..=(col - 1);
            schematic_numbers_in_row.insert(
                range.clone(),
                SchematicNumber {
                    value: parse_num(curr_num_chars),
                    row,
                    range,
                },
            );
        }

        schematic_numbers.push(schematic_numbers_in_row);
        row += 1;
    }

    // println!("Schematic numbers: {:?}", schematic_numbers);
    // println!("Symbol locations: {:?}", symbol_locations);
    // println!("Parsed {} rows, {} cols", n_rows, n_cols);

    let mut sum = 0;
    for star_loc in star_locations {
        let mut neighbor_part_nums = Vec::new();
        for neighbor in star_loc.neighbors(n_rows, n_cols) {
            let schematic_numbers_in_row = &mut schematic_numbers[neighbor.row];
            if let Some(found_number) = schematic_numbers_in_row.get(&neighbor.col) {
                neighbor_part_nums.push(found_number.clone());

                // Remove the part number so we don't count it twice for this neighbor
                // println!("Found {:?}", found_number);
                schematic_numbers_in_row.remove(found_number.range.clone());
            }
        }

        // Calculate the gear ratio, if any
        if neighbor_part_nums.len() == 2 {
            let gear_ratio = neighbor_part_nums[0].value * neighbor_part_nums[1].value;
            // println!("Adding gear ratio {gear_ratio}");
            sum += gear_ratio;
        }

        // Re-insert the part numbers so another gear can detect it
        for part_num in neighbor_part_nums {
            let clone = part_num.clone();
            schematic_numbers[part_num.row].insert(part_num.range, clone);
        }
    }
    println!("{}", sum);
}

fn is_number(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn parse_num(chars: Vec<char>) -> i32 {
    let num_str: String = chars.into_iter().collect();
    num_str.parse().expect("converting string to i32")
}
