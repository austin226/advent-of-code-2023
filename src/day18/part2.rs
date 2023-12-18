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
struct VectorPoint {
    coord: Coord,
    is_border: bool,
}

#[derive(Debug)]
struct VectorImage {
    points: HashMap<Coord, VectorPoint>,
    bottom_left: Coord,
    top_right: Coord,
}

impl VectorImage {
    fn new() -> Self {
        let start_coord = Self::start_coord();
        let mut new_svg = Self {
            points: HashMap::new(),
            bottom_left: start_coord,
            top_right: start_coord,
        };
        new_svg.add_border_point(start_coord);
        new_svg
    }

    fn start_coord() -> Coord {
        Coord::new(0, 0)
    }

    fn add_border_point(&mut self, coord: Coord) {
        // Expand bounding box
        self.bottom_left.x = std::cmp::min(self.bottom_left.x, coord.x);
        self.bottom_left.y = std::cmp::max(self.bottom_left.y, coord.y);
        self.top_right.x = std::cmp::max(self.top_right.x, coord.x);
        self.top_right.y = std::cmp::min(self.top_right.y, coord.y);

        // Store the point
        let point = VectorPoint {
            coord,
            is_border: true,
        };
        self.points.insert(coord, point);
    }

    fn add_interior_point(&mut self, coord: Coord) {
        let point = VectorPoint {
            coord,
            is_border: false,
        };
        self.points.insert(coord, point);
    }

    fn get_edge_neighbors(&self, coord: Coord) -> Vec<Coord> {
        use Direction::*;
        let mut neighbors = Vec::new();
        for direction in [U, R, D, L] {
            if let Some(neighbor) = self.points.get(&coord.next(direction)) {
                neighbors.push(neighbor.coord);
            }
        }
        debug_assert_eq!(2, neighbors.len(), "Polygon is not connected");
        neighbors
    }

    fn get_non_edge_neighbors(&self, coord: Coord) -> Vec<Coord> {
        use Direction::*;
        let mut neighbors = Vec::new();
        for direction in [U, R, D, L] {
            let neighbor_coord = coord.next(direction);
            if self.points.get(&neighbor_coord).is_none() {
                neighbors.push(neighbor_coord);
            }
        }
        neighbors
    }

    fn is_in_bounds(&self, coord: Coord) -> bool {
        (self.bottom_left.x..=self.top_right.x).contains(&coord.x)
            && (self.top_right.y..=self.bottom_left.y).contains(&coord.y)
    }

    /// Try to flood fill a polygon starting with a non-edge Coord.
    /// If we hit the edge of the bounding box, stop and return None.
    /// Otherwise, return all the coordinates inside the polygon.
    fn try_fill(&self, start: Coord) -> Option<HashSet<Coord>> {
        let mut visited = HashSet::<Coord>::new();
        visited.insert(start);
        let mut q: Queue<Coord> = queue![];
        let _ = q.add(start);
        while q.size() > 0 {
            let v = q.remove().unwrap();
            if !self.is_in_bounds(v) {
                // Out of bounds - not inside the polygon
                return None;
            }
            let neighbors = self.get_non_edge_neighbors(v);
            for u in neighbors {
                if !visited.contains(&u) {
                    visited.insert(u);
                    let _ = q.add(u);
                }
            }
        }

        Some(visited)
    }

    fn get_fill_coords(&self) -> HashSet<Coord> {
        assert!(self.points.len() >= 8, "Not enough points to fill polygon");

        // Advance until we're on an edge point (not a corner)
        let mut coord = self.points.get(&Self::start_coord()).unwrap().coord;
        let mut neighbors = self.get_edge_neighbors(coord);
        while coord.direction_to_neighbor(neighbors[0])
            != coord.direction_to_neighbor(neighbors[1]).turn_180()
        {
            // This is a corner
            coord = neighbors[0];
            neighbors = self.get_edge_neighbors(coord);
        }

        // 2 neighbors - this is on an edge
        debug_assert_eq!(2, neighbors.len());
        let neighbor = neighbors[0];
        let edge_dir = coord.direction_to_neighbor(neighbor);

        // Try filling in this direction
        // If we hit the edge of the bounding box, try the other potential direction
        let non_edge_dir = edge_dir.turn_90_cw();
        let non_edge_coord = coord.next(non_edge_dir);
        self.try_fill(non_edge_coord).unwrap_or_else(|| {
            // Try again with other non-edge coord
            let other_non_edge_coord = coord.next(non_edge_dir.turn_180());
            self.try_fill(other_non_edge_coord)
                .expect("Failed to fill in either direction")
        })
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
}

impl VectorPainter {
    fn new(start_coord: Coord) -> Self {
        Self {
            location: start_coord,
        }
    }

    fn paint(&mut self, step: &LineSegment, svg: &mut VectorImage) {
        for i in 0..step.distance {
            let next_coord = self.location.next(step.direction);
            svg.add_border_point(next_coord);
            self.location = next_coord;
        }
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

    let area = svg.points.len();
    println!("Area: {area}");
}
