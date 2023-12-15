use indicatif::ProgressBar;
use itertools::Itertools;

use crate::common::get_input;

// Assume input is rectangular
fn rotate_input_90cw(input: &mut Vec<String>) {
    if input.len() == 0 {
        return;
    }
    let w = input.len();
    let h = input[0].len();
    let mut output = vec![vec!['.'; h]; w];
    for r in 0..h {
        let in_chars = input[r].chars().collect_vec();
        for c in 0..w {
            output[c][h - 1 - r] = in_chars[c];
        }
    }
    let output = output
        .iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect_vec();
    *input = output;
}

fn shift_round_boulders(input: &mut Vec<String>) {
    let w = input.len();
    let h = input[0].len();
    let mut output = vec![vec!['.'; w]; h];
    for r in 0..h {
        let row = input[r].chars().collect_vec();
        let mut n_left = 0;
        for c in 0..=w {
            if c < w {
                output[r][c] = row[c];
            }
            if c == w || row[c] == '#' {
                // Shift boulders on the left
                for b in (c - n_left)..c {
                    output[r][b] = 'O';
                }
                for d in (0..(c - n_left)).rev() {
                    if output[r][d] == '#' {
                        break;
                    }
                    output[r][d] = '.';
                }
                n_left = 0;
            } else if row[c] == 'O' {
                n_left += 1;
            }
        }
    }
    let output = output
        .iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect_vec();
    *input = output;
}

fn calculate_load(input: Vec<String>) -> i32 {
    let w = input.len();
    let h = input[0].len();
    let mut load = 0i32;
    for r in 0..h {
        let row = input[r].chars().collect_vec();
        for c in 0..w {
            if row[c] == 'O' {
                load += (c + 1) as i32;
            }
        }
    }
    return load;
}

const GAP: u8 = 0;
const ROUND_BOULDER: u8 = 1;
const SQUARE_BOULDER: u8 = 2;

fn build_matrix(input: Vec<String>) -> Vec<u8> {
    let h = input.len();
    let w = input[0].len();
    let mut matrix = vec![GAP; w * h];

    for r in 0..h {
        let in_row_chars = input[r].chars().collect_vec();
        for c in 0..w {
            let in_char = in_row_chars[c];
            let mat_char = match in_char {
                'O' => ROUND_BOULDER,
                '#' => SQUARE_BOULDER,
                _ => continue,
            };
            matrix[r * w + c] = mat_char;
        }
    }
    return matrix;
}

fn rotate_matrix_90cw(matrix: &mut Vec<u8>) {

}

pub fn run() {
    let input = get_input("src/day14/input0.txt");

    const CYCLES: u64 = 1000000000;
    let bar = ProgressBar::new(CYCLES);
    let mut matrix = build_matrix(input);
    println!("{:?}", matrix);
    // let mut map: Vec<String> = input;

    // for _ in 0..CYCLES {
    //     rotate_input_90cw(&mut map);
    //     shift_round_boulders(&mut map);
    //     bar.inc(1);
    // }

    // bar.finish();
    // let load = calculate_load(map);
    // println!("{load}");
}
