use itertools::Itertools;

use crate::common::get_input;

fn extrapolate(vals: &Vec<i64>) -> i64 {
    // println!("vals are {:?}", vals);
    if vals.iter().all(|e| *e == 0) {
        // All elements are zero
        return 0;
    }

    if vals.len() == 1 {
        panic!("No solution");
    }

    // slopes will be 1 shorter than vals
    let slopes = (0..vals.len() - 1)
        .map(|i| vals[i + 1] - vals[i])
        .collect_vec();
    vals.last().unwrap() + extrapolate(&slopes)
}

pub fn run() {
    let input = get_input("src/day9/input0.txt");

    // values range from -100,000,000 to 100,000,000
    let result: i64 = input
        .iter()
        .map(|line| {
            let vals = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec();

            extrapolate(&vals)
        })
        .sum();
    println!("{result}");
}
