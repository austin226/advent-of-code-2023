use itertools::Itertools;
use crate::common::get_input;

enum Direction {
    U,
    R,
    D,
    L,
}

impl Direction {
    fn new(input: &str) -> Self {
        match input {
            "U" => Direction::U,
            "R" => Direction::R,
            "D" => Direction::D,
            "L" => Direction::L,
            _ => panic!("Bad direction {input}")
        }
    }
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(input: &str) -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

struct Step {
    direction: Direction,
    distance: i32,
    color: Color,
}

impl Step {
    fn new(input: String) -> Self {
        let tokens = input.split_ascii_whitespace().collect_vec();
        let direction = Direction::new(tokens[0]);
        let distance = tokens[1].parse::<i32>().unwrap();
        let color = Color::new(tokens[2]);
        Self { direction, distance, color }
    }
}

pub fn run() {
    let input = get_input("src/day18/input0.txt");
}
