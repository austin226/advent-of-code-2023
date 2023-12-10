use itertools::Itertools;

use crate::common::get_input;

#[derive(Copy, Clone, Debug)]
struct Tile {
    tile_type: TileType,
    point: Point,
}

impl Tile {
    fn is_start(&self) -> bool {
        self.tile_type == TileType::Start
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum TileType {
    Empty,
    Start,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl TileType {
    fn parse(c: char) -> Self {
        use TileType::*;
        match c {
            '.' => Empty,
            'S' => Start,
            '|' => NS,
            '-' => EW,
            'L' => NE,
            'J' => NW,
            '7' => SW,
            'F' => SE,
            _ => {
                panic!("Unknown tile symbol {c}");
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

pub fn run() {
    // Input is a square of pipe symbols
    let input = get_input("src/day10/input0.txt");

    let width = input[0].len();
    let height = input.len();

    // Parse tiles and get the starting point
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let mut starting_tile: Option<Tile> = None;
    for y in 0..width {
        let row_str = &input[y];
        let mut row = Vec::new();
        for x in 0..height {
            let tile = Tile {
                tile_type: TileType::parse(row_str.as_bytes()[x] as char),
                point: (Point { x: y, y: x }),
            };
            if tile.is_start() {
                starting_tile = Some(tile);
            }
            row.push(tile);
        }
        tiles.push(row);
    }

    let starting_tile = starting_tile.unwrap();
    println!("{:?}", tiles);
}
