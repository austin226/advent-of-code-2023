use itertools::Itertools;

use crate::common::get_input;

pub fn run() {
    let input = get_input("src/day6/input0.txt");

    let times_str = input[0].split_ascii_whitespace().collect_vec();
    let times = &times_str[1..times_str.len()]
        .iter()
        .map(|s| s.parse::<u8>().unwrap())
        .collect_vec();
    let dist_str = input[1].split_ascii_whitespace().collect_vec();
    let distances = &dist_str[1..dist_str.len()]
        .iter()
        .map(|s| s.parse::<u8>().unwrap())
        .collect_vec();
    println!("{:?}, {:?}", times, distances);
}
