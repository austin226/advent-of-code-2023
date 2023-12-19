use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Shl;

use itertools::Itertools;
use once_cell::sync::Lazy;
use queues::*;
use regex::Regex;

use crate::common::get_input;

const IN_FILE: &str = "src/day18/input0.txt";
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

    fn turn_90_cw(&self) -> Self {
        use Direction::*;
        match self {
            U => R,
            R => D,
            D => L,
            L => U,
        }
    }

    fn turn_180(&self) -> Self {
        use Direction::*;
        match self {
            U => D,
            R => L,
            D => U,
            L => R,
        }
    }

    /// Return 1 if it's a right-hand turn, -1 if it's a left-hand turn, or None if neither.
    fn turn_difference(&self, other: Direction) -> Option<i32> {
        use Direction::*;
        match (self, other) {
            (U, R) | (R, D) | (D, L) | (L, U) => Some(1),
            (U, L) | (L, D) | (D, R) | (R, U) => Some(-1),
            _ => None,
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
            Direction::U => Self::new(self.x, self.y - 1),
            Direction::R => Self::new(self.x + 1, self.y),
            Direction::D => Self::new(self.x, self.y + 1),
            Direction::L => Self::new(self.x - 1, self.y),
        }
    }

    fn direction_to_neighbor(&self, other: Coord) -> Direction {
        match (other.x - self.x, other.y - self.y) {
            (0, -1) => Direction::U,
            (1, 0) => Direction::R,
            (0, 1) => Direction::D,
            (-1, 0) => Direction::L,
            _ => panic!("{:?} is not a neighbor of {:?}", other, self),
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

    fn is_in_bounds(&self, coord: Coord) -> bool {
        (self.bottom_left.x..=self.top_right.x).contains(&coord.x)
            && (self.top_right.y..=self.bottom_left.y).contains(&coord.y)
    }
}

#[derive(Debug)]
struct LineSegment {
    direction: Direction,
    distance: i64,
}

impl LineSegment {
    fn new(input: &str) -> Self {
        let re = &LINE_REGEX;
        let caps = re.captures(input).unwrap();
        let dist_hex = caps.get(1).unwrap().as_str();
        let dist_hex = format!("0{}", dist_hex);
        let decoded_dist = hex::decode(dist_hex).unwrap();
        debug_assert_eq!(3, decoded_dist.len(), "Expected 3 hex bytes in dist");
        let distance: u64 = (decoded_dist[0] as u64).shl(16)
            + (decoded_dist[1] as u64).shl(8)
            + (decoded_dist[2] as u64);

        let dir_hex = caps.get(2).unwrap().as_str();
        let dir_hex = format!("0{}", dir_hex);
        let decoded_dir = hex::decode(dir_hex).unwrap();
        debug_assert_eq!(1, decoded_dir.len(), "Expected 1 hex byte in dir");

        let tokens = input.split_ascii_whitespace().collect_vec();
        let direction = Direction::new(decoded_dir[0]);

        Self {
            direction,
            distance: distance as i64,
        }
    }
}

struct VectorPainter {
    location: Coord,
    orientation: Option<Direction>,
    total_right_turns: i32,
}

impl VectorPainter {
    fn new(start_coord: Coord) -> Self {
        Self {
            location: start_coord,
            orientation: None,
            total_right_turns: 0,
        }
    }

    fn paint(&mut self, step: &LineSegment, svg: &mut VectorImage) {
        let next_coord = self.location.next(step.direction);
        svg.add_vertex(next_coord);
        self.location = next_coord;

        if let Some(prev) = self.orientation {
            self.total_right_turns += prev.turn_difference(step.direction).unwrap_or_else(|| {
                panic!(
                    "Invalid step - must turn 90 degrees. Started {:?} and went {:?}",
                    prev, step.direction,
                )
            });
        }
        self.orientation = Some(step.direction);
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
    // svg.fill_polygon(Color::new(FILL_COLOR));

    let area = svg.vertices.len();
    println!("Area: {area}");
}
