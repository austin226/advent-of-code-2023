use std::collections::HashSet;
use std::hash::Hash;

use itertools::Itertools;

use crate::common::get_input;

#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Hash, Copy, Clone, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize,
}

enum TileType {
    Start,
    GardenPlot,
    Rock,
}

impl TileType {
    fn new(chr: char) -> Self {
        match chr {
            'S' => TileType::Start,
            '.' => TileType::GardenPlot,
            '#' => TileType::Rock,
            _ => panic!(),
        }
    }
}

struct Tile {
    point: Point,
    tile_type: TileType,
}

struct Map {
    height: usize,
    width: usize,
    start: Point,
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

        let mut start_point: Option<Point> = None;
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        tiles.reserve(height);
        for r in 0..height {
            assert_eq!(width, input[0].len(), "Map must be rectangular");
            let row_chars = input[r].chars().collect_vec();

            let mut row = Vec::new();
            row.reserve(width);
            for c in 0..width {
                let point = Point { row: r, col: c };
                let chr = row_chars[c];
                let tile_type = TileType::new(chr);
                match tile_type {
                    TileType::Start => {
                        start_point = Some(point);
                    }
                    _ => {}
                }
                let tile = Tile { point, tile_type };
                row.push(tile);
            }
            tiles.push(row);
        }
        Self {
            width,
            height,
            start: start_point.expect("start point"),
            tiles,
        }
    }

    fn next_point(&self, start: &Point, direction: &Direction) -> Option<Point> {
        use Direction::*;
        match direction {
            N => {
                if start.row == 0 {
                    None
                } else {
                    Some(Point {
                        row: start.row - 1,
                        col: start.col,
                    })
                }
            }
            E => {
                if start.col == self.width - 1 {
                    None
                } else {
                    Some(Point {
                        row: start.row,
                        col: start.col + 1,
                    })
                }
            }
            S => {
                if start.row == self.height - 1 {
                    None
                } else {
                    Some(Point {
                        row: start.row + 1,
                        col: start.col,
                    })
                }
            }
            W => {
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

    fn step(&self, start_points: &HashSet<Point>) -> HashSet<Point> {
        use Direction::*;

        let mut res = HashSet::new();
        for point in start_points {
            for direction in [N, E, S, W] {
                if let Some(next_tile) = self.next_tile(point, &direction) {
                    match next_tile.tile_type {
                        TileType::Start | TileType::GardenPlot => {
                            res.insert(next_tile.point);
                        }
                        _ => {}
                    }
                }
            }
        }
        res
    }
}

pub fn run() {
    let input = get_input("src/day21/input0.txt");
    let map = Map::new(input);

    let mut points = HashSet::new();
    points.insert(map.start);

    const STEPS: i32 = 64;
    for i in 0..STEPS {
        points = map.step(&points);
    }
    println!("{}", points.len());
}
