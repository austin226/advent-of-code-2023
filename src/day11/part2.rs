use std::cmp::max;
use std::cmp::min;
use std::fmt;

use itertools::Itertools;

use crate::common::get_input;

const EXPANSION_SIZE: u64 = 1_000_000;

#[derive(Copy, Clone)]
struct Galaxy {
    // id: u32,
    row: u64,
    col: u64,
}

impl fmt::Debug for Galaxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

impl Galaxy {
    fn manhattan_distance(self, other: &Galaxy) -> u64 {
        (max(self.row, other.row) - min(self.row, other.row)) as u64
            + (max(self.col, other.col) - min(self.col, other.col)) as u64
    }
}

fn find_occupied_rows_cols(input: &Vec<String>) -> (Vec<bool>, Vec<bool>) {
    let map_width = input[0].len();
    let map_height = input.len();

    let mut occupied_cols = vec![false; map_width];
    let mut occupied_rows = vec![false; map_height];

    for r in 0..map_height {
        let row_chars = input.get(r).unwrap().chars().collect_vec();
        for c in 0..map_width {
            let cell = row_chars[c];
            if cell == '#' {
                // mark occupied rows and cols
                occupied_rows[r] = true;
                occupied_cols[c] = true;
            }
        }
    }

    return (occupied_rows, occupied_cols);
}

fn expanded_galaxy_locations(
    input: &Vec<String>,
    occupied_rows: &Vec<bool>,
    occupied_cols: &Vec<bool>,
) -> Vec<Galaxy> {
    // let mut gid = 1;
    let map_width = input[0].len();
    let map_height = input.len();

    let mut galaxies = Vec::<Galaxy>::new();

    let mut exp_r = 0;
    for r in 0..map_height {
        let mut exp_c = 0;
        if !occupied_rows[r] {
            exp_r += EXPANSION_SIZE - 1
        }
        let row_chars = input.get(r).unwrap().chars().collect_vec();
        for c in 0..map_width {
            if !occupied_cols[c] {
                exp_c += EXPANSION_SIZE - 1
            }
            let cell = row_chars[c];
            if cell == '#' {
                // Save this galaxy's cocrdinates
                galaxies.push(Galaxy {
                    // id: gid,
                    row: exp_r,
                    col: exp_c,
                });
                // gid += 1;
            }
            exp_c += 1;
        }
        exp_r += 1;
    }
    return galaxies;
}

pub fn run() {
    let input = get_input("src/day11/input1.txt");

    // First pass - determine expansion
    let (occupied_rows, occupied_cols) = find_occupied_rows_cols(&input);

    // Second pass - determing galaxy locations after expansion
    let galaxies = expanded_galaxy_locations(&input, &occupied_rows, &occupied_cols);

    // For each pair of galaxies, find the manhattan distance between them.
    let ans: u64 = (0..galaxies.len())
        .map(|i| {
            ((i + 1)..galaxies.len())
                .map(|j| galaxies[i].manhattan_distance(&galaxies[j]))
                .sum::<u64>()
        })
        .sum();

    println!("{ans}");
}
