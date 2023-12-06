use itertools::Itertools;

use crate::common::get_input;

pub fn run() {
    let input = get_input("src/day6/input1.txt");

    let times = {
        let times_str = input[0].split_ascii_whitespace().collect_vec();
        &times_str[1..times_str.len()]
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect_vec()
    };
    let distances = {
        let dist_str = input[1].split_ascii_whitespace().collect_vec();
        &dist_str[1..dist_str.len()]
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect_vec()
    };

    debug_assert_eq!(times.len(), distances.len());

    let mut product = 1;
    for i in 0..times.len() {
        let race_time = times[i];
        let record_dist = distances[i];

        let ways_to_win = (1..race_time)
            .map(|charge_time| (race_time - charge_time) as i64 * charge_time as i64)
            .filter(|&dist| dist > record_dist as i64)
            .count();

        product *= ways_to_win;
        println!("{ways_to_win} ways to win against time {race_time}");
    }

    println!("{:?}, {:?}", times, distances);
    println!("Product: {product}");
}
