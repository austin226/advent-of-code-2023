use bmp::{px, Image, Pixel};
use std::hash::Hash;

use itertools::{interleave, Itertools};

use crate::common::get_input;

const IN_FILE: &str = "src/day18/input0.txt";
const OUT_FILE: &str = "src/day18/output0.bmp";

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(input: &str) -> Self {
        let input = input.strip_prefix('#').unwrap_or(input);
        let decoded_string = hex::decode(input).unwrap();
        assert_eq!(3, decoded_string.len(), "Expected 3 hexes in RGB code");
        Self {
            r: decoded_string[0],
            g: decoded_string[1],
            b: decoded_string[2],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn next(&self, direction: Direction) -> Self {
        match direction {
            Direction::U => Self::new(self.x, self.y + 1),
            Direction::R => Self::new(self.x + 1, self.y),
            Direction::D => Self::new(self.x, self.y - 1),
            Direction::L => Self::new(self.x - 1, self.y),
        }
    }
}

#[derive(Debug)]
struct Map {
    points: Vec<(Color, Coord)>,
    top_left: Coord,
    bottom_right: Coord,
}

impl Map {
    fn new() -> Self {
        let start_coord = Self::start_coord();
        let mut new_map = Self {
            points: Vec::new(),
            top_left: start_coord,
            bottom_right: start_coord,
        };
        new_map.add_point(start_coord, Color::new("#000000"));
        new_map
    }

    fn start_coord() -> Coord {
        Coord::new(0, 0)
    }

    fn add_point(&mut self, coord: Coord, color: Color) {
        // Expand bounding box
        self.top_left.x = std::cmp::min(self.top_left.x, coord.x);
        self.top_left.y = std::cmp::max(self.top_left.y, coord.y);
        self.bottom_right.x = std::cmp::max(self.bottom_right.x, coord.x);
        self.bottom_right.y = std::cmp::min(self.bottom_right.y, coord.y);

        // Store the point
        self.points.push((color, coord));
    }

    fn rasterize(&self) -> Vec<Option<Color>> {
        assert!(self.bottom_right.x > self.top_left.x);
        assert!(self.top_left.y > self.bottom_right.y);
        let width = (self.bottom_right.x - self.top_left.x + 1) as usize;
        let height = (self.top_left.y - self.bottom_right.y + 1) as usize;

        let mut bitmap = vec![None; width * height];

        // Draw border
        for (color, coord) in self.points.iter() {
            assert!(coord.x >= self.top_left.x);
            assert!(coord.x <= self.bottom_right.x);
            assert!(coord.y <= self.top_left.y);
            assert!(coord.y >= self.bottom_right.y);

            let rasterized_x = (coord.x - self.top_left.x) as usize;
            let rasterized_y = (coord.y - self.bottom_right.y) as usize;
            let bitmap_index = rasterized_y * width + rasterized_x;
            bitmap[bitmap_index] = Some(color);
        }

        // Actually draw a bitmap for debugging
        let mut img = Image::new(width as u32, height as u32);
        for (x, y) in img.coordinates() {
            if let Some(color) = bitmap[x as usize + (y as usize) * width] {
                img.set_pixel(x, y, px!(color.r, color.g, color.b));
            }
        }
        let _ = img.save(OUT_FILE);
        // println!("{:?}", bitmap);

        // TODO rasterize with colors
        // TODO find area of interior
        // TODO
        vec![]
    }
}

#[derive(Debug)]
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

        let color = {
            let color_str = tokens[2];
            let color_str = color_str.strip_prefix('(').unwrap_or(color_str);
            let color_str = color_str.strip_suffix(')').unwrap_or(color_str);
            Color::new(color_str)
        };

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

    fn perform_step(&mut self, step: &Step, map: &mut Map) {
        for i in 0..(step.distance) {
            let next_coord = self.location.next(step.direction);
            map.add_point(next_coord, step.color);
            self.location = next_coord;
        }
    }
}

pub fn run() {
    let input = get_input(IN_FILE);

    let mut map = Map::new();
    let mut worker = Worker::new(Map::start_coord());
    let steps = input.iter().map(|line| line.as_str()).map(Step::new);
    for step in steps {
        worker.perform_step(&step, &mut map);
    }

    map.rasterize();

    // println!("{:?}", map);
}
