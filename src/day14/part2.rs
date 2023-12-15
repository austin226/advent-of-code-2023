use indicatif::ProgressBar;
use itertools::Itertools;

use crate::common::get_input;

const GAP: u8 = 0;
const ROUND_BOULDER: u8 = 1;
const SQUARE_BOULDER: u8 = 2;
const UNKNOWN: u8 = 8;

// Shift boulders from the left to the right. O(n^2).
fn shift_round_boulders(matrix: &mut Vec<u8>, n: usize) {
    for r in 0..n {
        let mut n_left = 0;
        for c in 0..=n {
            let cur = if c == n {
                UNKNOWN
            } else {
                matrix[to_mat_coord(r, c, n)]
            };
            if c == n || cur == SQUARE_BOULDER {
                // Shift boulders on the left
                for b in (c - n_left)..c {
                    matrix[to_mat_coord(r, b, n)] = ROUND_BOULDER;
                }
                for d in (0..(c - n_left)).rev() {
                    if matrix[to_mat_coord(r, d, n)] == SQUARE_BOULDER {
                        break;
                    }
                    matrix[to_mat_coord(r, d, n)] = GAP;
                }
                n_left = 0;
            } else if cur == ROUND_BOULDER {
                n_left += 1;
            }
        }
    }
}

fn calculate_load(matrix: &Vec<u8>, n: usize) -> i32 {
    let mut load = 0i32;
    for r in 0..n {
        for c in 0..n {
            let cur = matrix[to_mat_coord(r, c, n)];
            if cur == ROUND_BOULDER {
                load += (c + 1) as i32;
            }
        }
    }
    return load;
}

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
            let top_left = to_mat_coord(i, j, n);
            let top_right = to_mat_coord(j, n - 1 - i, n);
            let bottom_right = to_mat_coord(n - 1 - i, n - 1 - j, n);
            let bottom_left = to_mat_coord(n - 1 - j, i, n);

            let temp = matrix[top_left];
            matrix[top_left] = matrix[bottom_left];
            matrix[bottom_left] = matrix[bottom_right];
            matrix[bottom_right] = matrix[top_right];
            matrix[top_right] = temp;
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

    const CYCLES: u64 = 1_000_000_000;
    let bar = ProgressBar::new(CYCLES);
    let n = input.len();
    assert_ne!(n, 0, "Input must be non-empty");
    assert_eq!(n, input[0].len(), "Input must be square");

    let mut matrix = build_matrix(input, n);
    // print_matrix(&matrix, n);

    for _ in 0..(CYCLES) {
        for _ in 0..4 {
            rotate_matrix_90cw(&mut matrix, n);
            shift_round_boulders(&mut matrix, n);
        }
        bar.inc(1);
    }

    bar.finish();
    let load = calculate_load(&matrix, n);
    println!("{load}");
}
