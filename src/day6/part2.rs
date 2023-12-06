use itertools::Itertools;

use crate::common::get_input;

pub fn run() {
    let input = get_input("src/day6/input1.txt");

    let race_time = input[0].split(":").collect_vec()[1]
        .replace(" ", "")
        .parse::<i32>()
        .unwrap();
    let record_dist = input[1].split(":").collect_vec()[1]
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    let ways_to_win = (1..race_time)
        .map(|charge_time| (race_time - charge_time) as i64 * charge_time as i64)
        .filter(|&dist| dist > record_dist)
        .count();

    println!("Ways to win: {ways_to_win}");
}
