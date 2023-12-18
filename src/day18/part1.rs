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
            _ => panic!("Bad direction {input}"),
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

struct Coord {
    x: i32,
    y: i32,
}

struct Tile {
    coord: Coord,
    color: Color,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn new() -> Self {
        Self { tiles: Vec::new() }
    }
}

struct Step {
    direction: Direction,
    distance: i32,
    color: Color,
}

impl Step {
    fn new(input: &str) -> Self {
        let tokens = input.split_ascii_whitespace().collect_vec();
        let direction = Direction::new(tokens[0]);
        let distance = tokens[1].parse::<i32>().unwrap();
        let color = Color::new(tokens[2]);
        Self {
            direction,
            distance,
            color,
        }
    }
}

struct Worker {
    location: Coord,
}

impl Worker {
    fn new() -> Self {
        Self {
            location: Coord { x: 0, y: 0 },
        }
    }

    fn perform_step(&self, step: &Step, map: &mut Map) {}
}

pub fn run() {
    let input = get_input("src/day18/input0.txt");

    let worker = Worker::new();
    let map = Map::new();
    let steps = input.iter().map(|line| line.as_str()).map(Step::new);

    for step in steps {}
}
