use crate::common::get_input;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug)]
struct Brick {
    ends: (Point, Point),
}

impl Brick {
    fn parse(line: &str) -> Self {
        static MODULE_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap());
        let caps = MODULE_REGEX.captures(line).unwrap();
        let nums =
            [1, 2, 3, 4, 5, 6].map(|i| caps.get(i).unwrap().as_str().parse::<i32>().unwrap());
        Self {
            ends: (
                Point::new(nums[0], nums[1], nums[2]),
                Point::new(nums[3], nums[4], nums[5]),
            ),
        }
    }
}

pub fn run() {
    let input = get_input("src/day22/input0.txt");
    let bricks = input
        .iter()
        .map(|line| Brick::parse(line.as_str()))
        .collect_vec();
    println!("{:?}", bricks);
}
