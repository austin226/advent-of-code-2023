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

fn build_matrix(input: Vec<String>, n: usize) -> Vec<u8> {
    let mut matrix = vec![GAP; n * n];

    for r in 0..n {
        let in_row_chars = input[r].chars().collect_vec();
        for c in 0..n {
            let in_char = in_row_chars[c];
            let mat_char = match in_char {
                'O' => ROUND_BOULDER,
                '#' => SQUARE_BOULDER,
                _ => continue,
            };
            matrix[to_mat_coord(r, c, n)] = mat_char;
        }
    }
    return matrix;
}

fn to_mat_coord(r: usize, c: usize, n: usize) -> usize {
    r * n + c
}

// Rotate the matrix in-place, O(n^2) time
fn rotate_matrix_90cw(matrix: &mut Vec<u8>, n: usize) {
    for i in 0..(n / 2) {
        for j in i..(n - i - 1) {
            let a = to_mat_coord(i, j, n);
            let b = to_mat_coord(n - 1 - j, i, n);
            let c = to_mat_coord(n - 1 - i, n - 1 - j, n);
            let d = to_mat_coord(j, n - 1 - i, n);

            let temp = matrix[i * n + j];
            matrix[a] = matrix[b];
            matrix[b] = matrix[c];
            matrix[c] = matrix[d];
            matrix[d] = temp;
        }
    }
}

fn print_matrix(matrix: &Vec<u8>, n: usize) {
    for i in 0..n {
        for j in 0..n {
            let c = match matrix[i * n + j] {
                GAP => '.',
                ROUND_BOULDER => 'O',
                SQUARE_BOULDER => '#',
                _ => panic!(),
            };
            print!("{c}");
        }
        println!();
    }
    println!();
}

pub fn run() {
    let input = get_input("src/day14/input0.txt");

    const CYCLES: u64 = 1000000000;
    let bar = ProgressBar::new(CYCLES);
    let n = input.len();
    assert_ne!(n, 0, "Input must be non-empty");
    assert_eq!(n, input[0].len(), "Input must be square");

    let mut matrix = build_matrix(input, n);
    print_matrix(&matrix, n);

    rotate_matrix_90cw(&mut matrix, n);
    print_matrix(&matrix, n);
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
