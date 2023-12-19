use std::hash::Hash;
use std::ops::Shl;

use itertools::Itertools;
use num::abs;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::common::get_input;

const IN_FILE: &str = "src/day18/input1.txt";
static LINE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r".*\(#(.....)(.)\)").unwrap());

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    U,
    R,
    D,
    L,
}

impl Direction {
    fn new(input: u8) -> Self {
        match input {
            0 => Direction::R,
            1 => Direction::D,
            2 => Direction::L,
            3 => Direction::U,
            _ => panic!("Bad direction {input}"),
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

    fn next(&self, direction: Direction, distance: i32) -> Self {
        match direction {
            Direction::U => Self::new(self.x, self.y - distance),
            Direction::R => Self::new(self.x + distance, self.y),
            Direction::D => Self::new(self.x, self.y + distance),
            Direction::L => Self::new(self.x - distance, self.y),
        }
    }
}

#[derive(Debug)]
struct VectorImage {
    vertices: Vec<Coord>,
    bottom_left: Coord,
    top_right: Coord,
}

impl VectorImage {
    fn new() -> Self {
        let start_coord = Self::start_coord();
        Self {
            vertices: vec![start_coord],
            bottom_left: start_coord,
            top_right: start_coord,
        }
    }

    fn start_coord() -> Coord {
        Coord::new(0, 0)
    }

    fn add_vertex(&mut self, coord: Coord) {
        // Expand bounding box
        self.bottom_left.x = std::cmp::min(self.bottom_left.x, coord.x);
        self.bottom_left.y = std::cmp::max(self.bottom_left.y, coord.y);
        self.top_right.x = std::cmp::max(self.top_right.x, coord.x);
        self.top_right.y = std::cmp::min(self.top_right.y, coord.y);

        // Store the point
        self.vertices.push(coord);
    }

    fn area(&self) -> u64 {
        // Calculate interior area - see
        // https://cp-algorithms.com/geometry/area-of-simple-polygon.html
        let mut res: i64 = 0;
        for i in 0..self.vertices.len() {
            let p = if i > 0 {
                self.vertices[i - 1]
            } else {
                *self.vertices.last().expect("last item in vec")
            };
            let q = self.vertices[i];

            // Add the area from the interior
            res += (p.x - q.x) as i64 * (p.y + q.y) as i64;

            // Add the area of the border tiles
            res += abs(p.x - q.x) as i64;
            res += abs(p.y - q.y) as i64;
        }

        // Add 1 for the start tile
        (abs(res) / 2 + 1) as u64
    }
}

#[derive(Debug)]
struct LineSegment {
    direction: Direction,
    distance: u32,
}

impl LineSegment {
    fn new(input: &str) -> Self {
        let re = &LINE_REGEX;
        let caps = re.captures(input).unwrap();
        let dist_hex = caps.get(1).unwrap().as_str();
        let dist_hex = format!("0{}", dist_hex);
        let decoded_dist = hex::decode(dist_hex).unwrap();
        debug_assert_eq!(3, decoded_dist.len(), "Expected 3 hex bytes in dist");
        let distance: u32 = (decoded_dist[0] as u32).shl(16)
            + (decoded_dist[1] as u32).shl(8)
            + (decoded_dist[2] as u32);

        let dir_hex = caps.get(2).unwrap().as_str();
        let dir_hex = format!("0{}", dir_hex);
        let decoded_dir = hex::decode(dir_hex).unwrap();
        debug_assert_eq!(1, decoded_dir.len(), "Expected 1 hex byte in dir");

        let tokens = input.split_ascii_whitespace().collect_vec();
        let direction = Direction::new(decoded_dir[0]);

        Self {
            direction,
            distance,
        }
    }
}

struct VectorPainter {
    location: Coord,
}

impl VectorPainter {
    fn new(start_coord: Coord) -> Self {
        Self {
            location: start_coord,
        }
    }

    fn paint(&mut self, step: &LineSegment, svg: &mut VectorImage) {
        let next_coord = self.location.next(step.direction, step.distance as i32);
        svg.add_vertex(next_coord);
        self.location = next_coord;
    }
}

pub fn run() {
    let input = get_input(IN_FILE);

    // Assume data draws a polygon that does not intersect with itself, and
    // that no two edge segments are touching.
    let mut svg = VectorImage::new();
    let mut painter = VectorPainter::new(VectorImage::start_coord());
    input
        .iter()
        .map(|line| line.as_str())
        .map(LineSegment::new)
        .for_each(|seg| {
            painter.paint(&seg, &mut svg);
        });

    let area = svg.area();
    println!("Area: {area}");
}
