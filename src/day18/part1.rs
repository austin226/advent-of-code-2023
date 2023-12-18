use std::collections::HashMap;
use std::hash::Hash;

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
        // TODO
        Self { r: 0, g: 0, b: 0 }
    }
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn next(&self, direction: &Direction) -> Self {
        match direction {
            Direction::U => Self::new(self.x, self.y + 1),
            Direction::R => Self::new(self.x + 1, self.y),
            Direction::D => Self::new(self.x, self.y - 1),
            Direction::L => Self::new(self.x - 1, self.y),
        }
    }
}

struct Tile {
    coord: Coord,
    color: Color,
}

struct Map {
    tiles: HashMap<Coord, Tile>,
}

impl Map {
    fn new() -> Self {
        let mut hash_map = HashMap::new();
        let start_tile = Tile {
            coord: Self::start_coord(),
            color: Color::new("#000000"),
        };
        hash_map.insert(Self::start_coord(), start_tile);
        Self { tiles: hash_map }
    }

    fn start_coord() -> Coord {
        Coord::new(0, 0)
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
    fn new(start_coord: Coord) -> Self {
        Self {
            location: start_coord,
        }
    }

    fn perform_step(&self, step: &Step, map: &mut Map) {
        // let tiles_in_step = Vec::new();
        // for i in 0..(step.distance) {
        //
        // }
    }
}

pub fn run() {
    let input = get_input("src/day18/input0.txt");

    let mut map = Map::new();
    let worker = Worker::new(Map::start_coord());
    let steps = input.iter().map(|line| line.as_str()).map(Step::new);
    for step in steps {
        worker.perform_step(&step, &mut map);
    }
}
