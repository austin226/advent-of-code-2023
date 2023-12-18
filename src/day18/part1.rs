use std::hash::Hash;

use bmp::{px, Image, Pixel};
use itertools::Itertools;

use crate::common::get_input;

const IN_FILE: &str = "src/day18/input0.txt";
const OUT_FILE: &str = "src/day18/output0.bmp";

const DEFAULT_COLOR: &str = "#000000";

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
            Direction::U => Self::new(self.x, self.y - 1),
            Direction::R => Self::new(self.x + 1, self.y),
            Direction::D => Self::new(self.x, self.y + 1),
            Direction::L => Self::new(self.x - 1, self.y),
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
struct VectorImage {
    points: Vec<(Color, Coord)>,
    bottom_left: Coord,
    top_right: Coord,
}

impl VectorImage {
    fn new() -> Self {
        let start_coord = Self::start_coord();
        let mut new_svg = Self {
            points: Vec::new(),
            bottom_left: start_coord,
            top_right: start_coord,
        };
        new_svg.add_point(start_coord, Color::new(DEFAULT_COLOR));
        new_svg
    }

    fn start_coord() -> Coord {
        Coord::new(0, 0)
    }

    fn add_point(&mut self, coord: Coord, color: Color) {
        // Expand bounding box
        self.bottom_left.x = std::cmp::min(self.bottom_left.x, coord.x);
        self.bottom_left.y = std::cmp::max(self.bottom_left.y, coord.y);
        self.top_right.x = std::cmp::max(self.top_right.x, coord.x);
        self.top_right.y = std::cmp::min(self.top_right.y, coord.y);

        // Store the point
        self.points.push((color, coord));
    }

    fn rasterize(&self) -> Bitmap {
        assert!(self.top_right.x > self.bottom_left.x);
        assert!(self.bottom_left.y > self.top_right.y);
        let width = (self.top_right.x - self.bottom_left.x + 1) as usize;
        let height = (self.bottom_left.y - self.top_right.y + 1) as usize;

        let mut bitmap = vec![None; width * height];

        // Draw border
        for (color, coord) in self.points.iter() {
            assert!(coord.x >= self.bottom_left.x);
            assert!(coord.x <= self.top_right.x);
            assert!(coord.y <= self.bottom_left.y);
            assert!(coord.y >= self.top_right.y);

            let rasterized_x = (coord.x - self.bottom_left.x) as usize;
            let rasterized_y = (coord.y - self.top_right.y) as usize;
            let bitmap_index = rasterized_y * width + rasterized_x;
            bitmap[bitmap_index] = Some(*color);
        }

        Bitmap {
            pixels: bitmap,
            width: width as u32,
            height: height as u32,
        }
        // TODO find area of interior
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
        for i in 0..(step.distance) {
            let next_coord = self.location.next(step.direction);
            svg.add_point(next_coord, step.color);
            self.location = next_coord;
        }
    }
}

pub fn run() {
    let input = get_input(IN_FILE);

    let mut svg = VectorImage::new();
    let mut worker = VectorDrawer::new(VectorImage::start_coord());
    input
        .iter()
        .map(|line| line.as_str())
        .map(LineSegment::new)
        .for_each(|step| {
            worker.perform_step(&step, &mut svg);
        });
    let bitmap = svg.rasterize();
    bitmap.render(OUT_FILE);

    // println!("{:?}", map);
}
