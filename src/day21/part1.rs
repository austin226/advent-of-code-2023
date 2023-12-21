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

#[derive(Hash, Copy, Clone)]
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
}

pub fn run() {
    let input = get_input("src/day21/input0.txt");
    let map = Map::new(input);
}
