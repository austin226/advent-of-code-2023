use itertools::Itertools;

use crate::common::get_input;

fn slopes(vals: &Vec<i32>) {}

pub fn run() {
    let input = get_input("src/day9/input0.txt");

    // values range from -100,000,000 to 100,000,000
    for line in input {
        let vals = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect_vec();

        println!("{:?}", vals);
    }
}
