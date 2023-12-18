use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use bmp::{px, Image, Pixel};
use itertools::Itertools;
use queues::*;

use crate::common::get_input;

const IN_FILE: &str = "src/day18/input1.txt";
const OUT_FILE: &str = "src/day18/output1.bmp";

const DEFAULT_COLOR: &str = "#000000";
const FILL_COLOR: &str = "#ffffff";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

struct Bitmap {
    width: u32,
    height: u32,
    pixels: Vec<Option<Color>>,
}

impl Bitmap {
    fn render(&self, output_file: &str) {
        let mut img = Image::new(self.width, self.height);
        for (x, y) in img.coordinates() {
            if let Some(color) = self.pixels[x as usize + (y * self.width) as usize] {
                img.set_pixel(x, y, px!(color.r, color.g, color.b));
            }
        }
        let _ = img.save(output_file);
    }
}

#[derive(Debug)]
struct VectorPoint {
    coord: Coord,
    color: Color,
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
        new_svg.add_border_point(start_coord, Color::new(DEFAULT_COLOR));
        new_svg
    }

    fn start_coord() -> Coord {
        Coord::new(0, 0)
    }

    fn add_border_point(&mut self, coord: Coord, color: Color) {
        // Expand bounding box
        self.bottom_left.x = std::cmp::min(self.bottom_left.x, coord.x);
        self.bottom_left.y = std::cmp::max(self.bottom_left.y, coord.y);
        self.top_right.x = std::cmp::max(self.top_right.x, coord.x);
        self.top_right.y = std::cmp::min(self.top_right.y, coord.y);

        // Store the point
        let point = VectorPoint {
            coord,
            color,
            is_border: true,
        };
        self.points.insert(coord, point);
    }

    fn add_interior_point(&mut self, coord: Coord, color: Color) {
        let point = VectorPoint {
            coord,
            color,
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

    /// Try to fill a polygon starting with a non-edge Coord.
    /// If we hit the edge of the bounding box, stop and return None.
    /// Otherwise, return all the coordinates inside the polygon.
    fn try_fill(&self, start: Coord) -> Option<Vec<Coord>> {
        let mut res = Vec::new();

        let mut visited = HashSet::<Coord>::new();
        let mut q: Queue<Coord> = queue![];
        let _ = q.add(start);
        while q.size() > 0 {
            let v = q.remove().unwrap();
            res.push(v);
            visited.insert(v);
            for u in self.get_non_edge_neighbors(v) {
                if !self.is_in_bounds(u) {
                    // Out of bounds - not inside the polygon
                    return None;
                }
                if !visited.contains(&u) {
                    let _ = q.add(u);
                }
            }
        }

        Some(res)
    }

    fn get_fill_coords(&self) -> Vec<Coord> {
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

    fn fill_polygon(&mut self, fill_color: Color) {
        let fill_coords = self.get_fill_coords();
        for f in fill_coords {
            self.add_interior_point(f, fill_color);
        }
    }

    fn rasterize(&self) -> Bitmap {
        assert!(self.top_right.x > self.bottom_left.x);
        assert!(self.bottom_left.y > self.top_right.y);
        let width = (self.top_right.x - self.bottom_left.x + 1) as usize;
        let height = (self.bottom_left.y - self.top_right.y + 1) as usize;

        let mut bitmap = vec![None; width * height];

        // Draw border
        for (coord, vector_point) in self.points.iter() {
            assert!(coord.x >= self.bottom_left.x);
            assert!(coord.x <= self.top_right.x);
            assert!(coord.y <= self.bottom_left.y);
            assert!(coord.y >= self.top_right.y);

            let rasterized_x = (coord.x - self.bottom_left.x) as usize;
            let rasterized_y = (coord.y - self.top_right.y) as usize;
            let bitmap_index = rasterized_y * width + rasterized_x;
            bitmap[bitmap_index] = Some(vector_point.color);
        }

        Bitmap {
            pixels: bitmap,
            width: width as u32,
            height: height as u32,
        }
    }
}

#[derive(Debug)]
struct LineSegment {
    direction: Direction,
    distance: i32,
    color: Color,
}

impl LineSegment {
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

struct VectorDrawer {
    location: Coord,
}

impl VectorDrawer {
    fn new(start_coord: Coord) -> Self {
        Self {
            location: start_coord,
        }
    }

    fn perform_step(&mut self, step: &LineSegment, svg: &mut VectorImage) {
        for i in 0..step.distance {
            let next_coord = self.location.next(step.direction);
            svg.add_border_point(next_coord, step.color);
            self.location = next_coord;
        }
    }
}

pub fn run() {
    let input = get_input(IN_FILE);

    // Assume data draws a polygon that does not intersect with itself, and
    // that no two edge segments are touching.
    let mut svg = VectorImage::new();
    let mut worker = VectorDrawer::new(VectorImage::start_coord());
    input
        .iter()
        .map(|line| line.as_str())
        .map(LineSegment::new)
        .for_each(|step| {
            worker.perform_step(&step, &mut svg);
        });
    svg.fill_polygon(Color::new(FILL_COLOR));

    // let bitmap = svg.rasterize();
    // bitmap.render(OUT_FILE);

    let area = svg.points.len();
    println!("Area: {area}");
}
