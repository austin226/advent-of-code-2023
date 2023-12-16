use core::fmt;
use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::common::get_input;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: u8,
    col: u8,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
enum MirrorDirection {
    LeanRight,
    LeanLeft,
}

#[derive(Clone, Copy)]
enum SplitterDirection {
    Horizontal,
    Vertical,
}

enum TileType {
    Empty,
    Mirror(MirrorDirection),
    Splitter(SplitterDirection),
}

impl TileType {
    fn new(chr: char) -> Self {
        match chr {
            '.' => TileType::Empty,
            '/' => TileType::Mirror(MirrorDirection::LeanRight),
            '\\' => TileType::Mirror(MirrorDirection::LeanLeft),
            '|' => TileType::Splitter(SplitterDirection::Vertical),
            '-' => TileType::Splitter(SplitterDirection::Horizontal),
            _ => panic!("Unknown tile type '{chr}'"),
        }
    }
}

struct Tile {
    point: Point,
    tile_type: TileType,
}

impl Tile {
    fn new(chr: char, row: u8, col: u8) -> Self {
        let tile_type = TileType::new(chr);
        let point = Point { row, col };
        Self { tile_type, point }
    }
}

struct Map {
    width: u8,
    height: u8,
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn new(input: Vec<String>) -> Self {
        assert!(
            input.len() > 0 && input[0].len() > 0,
            "Map must be at least 1x1"
        );
        let height = input.len();
        let width = input[0].len();
        assert!(
            height <= 256 && width <= 256,
            "Map must be 256x256 or smaller"
        );

        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        tiles.reserve(height);
        for r in 0..height {
            assert_eq!(width, input[0].len(), "Map must be rectangular");
            let row_chars = input[r].chars().collect_vec();

            let mut row = Vec::new();
            row.reserve(width);
            for c in 0..width {
                let chr = row_chars[c];
                row.push(Tile::new(chr, r as u8, c as u8));
            }
            tiles.push(row);
        }
        Self {
            width: width as u8,
            height: height as u8,
            tiles,
        }
    }

    fn next_point(&self, start: &Point, direction: &Direction) -> Option<Point> {
        use Direction::*;
        match direction {
            Up => {
                if start.row == 0 {
                    None
                } else {
                    Some(Point {
                        row: start.row - 1,
                        col: start.col,
                    })
                }
            }
            Right => {
                if start.col == self.width - 1 {
                    None
                } else {
                    Some(Point {
                        row: start.row,
                        col: start.col + 1,
                    })
                }
            }
            Down => {
                if start.row == self.height - 1 {
                    None
                } else {
                    Some(Point {
                        row: start.row + 1,
                        col: start.col,
                    })
                }
            }
            Left => {
                if start.col == 0 {
                    None
                } else {
                    Some(Point {
                        row: start.row,
                        col: start.col - 1,
                    })
                }
            }
        }
    }

    fn tile_at(&self, point: &Point) -> Option<&Tile> {
        self.tiles.get(point.row as usize)?.get(point.col as usize)
    }

    fn next_tile(&self, start: &Point, direction: &Direction) -> Option<&Tile> {
        let next_point = self.next_point(start, direction)?;
        self.tile_at(&next_point)
    }
}

struct Beam {
    id: u64,
    point: Point,
    direction: Direction,
    alive: bool,
}

impl Default for Beam {
    fn default() -> Self {
        Self::new(0, Point { row: 0, col: 0 }, Direction::Right)
    }
}

impl Beam {
    fn new(id: u64, point: Point, direction: Direction) -> Self {
        Self {
            id,
            point,
            direction,
            alive: true,
        }
    }

    /// Transform this beam, and maybe return a spawned beam as well
    fn transform(&mut self, map: &Map) -> Option<Beam> {
        let current_tile = map.tile_at(&self.point)?;
        let mut new_beam: Option<Beam> = None;
        match current_tile.tile_type {
            TileType::Empty => {
                // Continue in same direction
            }
            TileType::Mirror(mirror_direction) => {
                use Direction::*;
                use MirrorDirection::*;
                self.direction = match (self.direction, mirror_direction) {
                    (Up, LeanRight) => Right,
                    (Up, LeanLeft) => Left,
                    (Right, LeanRight) => Up,
                    (Right, LeanLeft) => Down,
                    (Down, LeanRight) => Left,
                    (Down, LeanLeft) => Right,
                    (Left, LeanRight) => Down,
                    (Left, LeanLeft) => Up,
                };
            }
            TileType::Splitter(splitter_direction) => {
                use Direction::*;
                use SplitterDirection::*;
                match (self.direction, splitter_direction) {
                    (Up, Vertical)
                    | (Right, Horizontal)
                    | (Down, Vertical)
                    | (Left, Horizontal) => {
                        // Continue in same direction
                    }
                    (Up, Horizontal) | (Down, Horizontal) => {
                        // Split left/right
                        self.direction = Direction::Left;
                        new_beam = Some(self.spawn_new_beam(Direction::Right));
                    }
                    (Right, Vertical) | (Left, Vertical) => {
                        // Split up/down
                        self.direction = Direction::Up;
                        new_beam = Some(self.spawn_new_beam(Direction::Down));
                    }
                }
            }
        }

        if let Some(point) = map.next_point(&self.point, &self.direction) {
            self.point = point;
        } else {
            self.die();
        }

        return new_beam;
    }

    fn spawn_new_beam(&self, direction: Direction) -> Beam {
        Beam::new(self.id + 1, self.point, direction)
    }

    fn die(&mut self) {
        println!("Beam {} dies", self.id);
        self.alive = false;
    }
}

pub fn run() {
    let input = get_input("src/day16/input1.txt");

    let map = Map::new(input);
    let mut visited_points = HashSet::<Point>::new();
    let mut visited_points_dirs = HashSet::<(Point, Direction)>::new();
    let mut beam_q = VecDeque::<Beam>::new();
    beam_q.push_back(Beam::default());

    while !beam_q.is_empty() {
        let mut beam = beam_q
            .pop_front()
            .expect("Tried to remove a beam from an empty queue");
        if !beam.alive {
            continue;
        }
        visited_points.insert(beam.point);
        visited_points_dirs.insert((beam.point, beam.direction));

        println!(
            "Beam {} starts at {} going {:?}",
            beam.id, beam.point, beam.direction
        );
        if let Some(split_beam) = beam.transform(&map) {
            println!("Beam {} spawns Beam {}", beam.id, split_beam.id);
            beam_q.push_back(split_beam);
        }
        println!(
            "Beam {} moves to {} going {:?}",
            beam.id, beam.point, beam.direction
        );

        if beam.alive {
            if visited_points_dirs.contains(&(beam.point, beam.direction)) {
                // Already visited this point with the same direction. Kill the beam.
                beam.die();
            } else {
                // Keep simulating this beam
                beam_q.push_back(beam);
            }
        }
    }

    println!("{}", visited_points.len());
}
