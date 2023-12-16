use itertools::Itertools;

use crate::common::get_input;

struct Point {
    row: u8,
    col: u8,
}

enum MirrorDirection {
    Right,
    Left,
}

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
            '/' => TileType::Mirror(MirrorDirection::Right),
            '\\' => TileType::Mirror(MirrorDirection::Left),
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
    width: usize,
    height: usize,
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
            width,
            height,
            tiles,
        }
    }
}

struct Beam {
    point: Point,
}

pub fn run() {
    let input = get_input("src/day16/input0.txt");

    let map = Map::new(input);
}
