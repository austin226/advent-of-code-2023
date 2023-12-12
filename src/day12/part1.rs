use indicatif::ProgressBar;
use itertools::Itertools;

use crate::common::get_input;

pub fn run() {
    let input = get_input("src/day12/input0.txt");
    let pb = ProgressBar::new(input.len() as u64);

    for line in input.iter() {
        let tokens = line.split_ascii_whitespace().collect_vec();
        let template = tokens[0];
        let nums: Vec<usize> = tokens[1]
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect_vec();

        println!("{}", template);
        println!("{:?}", nums);
    }

    pb.finish();
    println!("0");
}
