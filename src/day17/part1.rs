use itertools::Itertools;

use crate::common::get_input;

pub fn run() {
    let input = get_input("src/day17/input0.txt");
    let matrix: Vec<Vec<u8>> = input.iter().map(|row| {
        row.chars().map(|c| {
            c.to_digit(10).expect("Parsing a digit") as u8
        }).collect_vec()
    }).collect_vec();

    println!("{:?}", matrix);
}
