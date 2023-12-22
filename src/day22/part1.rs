use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::common::get_input;

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

#[derive(Debug)]
struct Tower {
    max_x: i32,
    max_y: i32,
    max_z: i32,
    bricks: Vec<Brick>,
}

impl Tower {
    fn new(bricks: Vec<Brick>) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;
        for brick in bricks.iter() {
            max_x = i32::max(max_x, brick.ends.0.x);
            max_x = i32::max(max_x, brick.ends.1.x);
            max_y = i32::max(max_y, brick.ends.0.y);
            max_y = i32::max(max_y, brick.ends.1.y);
            max_z = i32::max(max_z, brick.ends.0.z);
            max_z = i32::max(max_z, brick.ends.1.z);
        }

        Self {
            max_x,
            max_y,
            max_z,
            bricks,
        }
    }
}

pub fn run() {
    let input = get_input("src/day22/input0.txt");
    let bricks = input
        .iter()
        .map(|line| Brick::parse(line.as_str()))
        .collect_vec();
    let tower = Tower::new(bricks);
    println!("{:?}", tower);
}
