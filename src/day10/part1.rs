use itertools::Itertools;

use crate::common::get_input;

#[derive(Copy, Clone, Debug)]
struct Tile {
    tile_type: TileType,
    point: Point,
}

impl Tile {
    fn neighbor_points(&self, map_width: usize, map_height: usize) -> Vec<Point> {
        use TileType::*;
        let dxs = match self.tile_type {
            NS => vec![0, 0],
            EW => vec![1, -1],
            NE => vec![0, 1],
            NW => vec![0, -1],
            SW => vec![0, -1],
            SE => vec![0, 1],
            _ => vec![],
        };
        let dys = match self.tile_type {
            NS => vec![-1, 1],
            EW => vec![0, 0],
            NE => vec![-1, 0],
            NW => vec![-1, 0],
            SW => vec![1, 0],
            SE => vec![1, 0],
            _ => vec![],
        };
        debug_assert_eq!(dxs.len(), dys.len());

        let mut res = Vec::new();
        for i in 0..dxs.len() {
            let dx = dxs[i];
            let dy = dys[i];
            if let Some(neighbor_point) = self.point_in_dir(dy, dx, map_width, map_height) {
                res.push(neighbor_point);
            }
        }
        res
    }

    fn point_in_dir(&self, dy: i32, dx: i32, map_width: usize, map_height: usize) -> Option<Point> {
        let x = self.point.x as i32 + dx;
        let y = self.point.y as i32 + dy;
        if (0..map_width as i32).contains(&x) && (0..map_height as i32).contains(&y) {
            return Some(Point {
                x: x as usize,
                y: y as usize,
            });
        }
        return None;
    }

    fn resolve_starting_tile(&mut self, neighbors_nesw: [bool; 4]) {
        if !self.tile_type.is_start() {
            panic!("resolve_starting_tile called on non-starting tile");
        }

        use TileType::*;
        let new_type = match neighbors_nesw {
            [true, true, false, false] => NE,
            [true, false, true, false] => NS,
            [true, false, false, true] => NW,
            [false, true, true, false] => SE,
            [false, true, false, true] => EW,
            [false, false, true, true] => SW,
            _ => {
                panic!("Invalid neighbors_nesw: {:?}", neighbors_nesw);
            }
        };
        self.tile_type = new_type;
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

    fn is_start(&self) -> bool {
        *self == TileType::Start
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
    let mut starting_tile_point: Option<Point> = None;
    for y in 0..width {
        let row_str = &input[y];
        let mut row = Vec::new();
        for x in 0..height {
            let mut is_start = false;
            let tile_type = TileType::parse(row_str.as_bytes()[x] as char);
            let tile = Tile {
                tile_type,
                point: (Point { x, y }),
            };
            if tile_type.is_start() {
                starting_tile_point = Some(tile.point);
                is_start = true;
            }
            row.push(tile);
        }
        tiles.push(row);
    }

    // Determine starting tile directions based on neighboring points
    let starting_tile_point = starting_tile_point.unwrap();
    let mut starting_tile = tiles[starting_tile_point.y][starting_tile_point.x];
    let starting_neighbors_nesw = [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .map(|(dx, dy)| starting_tile.point_in_dir(dy, dx, width, height))
        .map(|p| {
            p.is_some() && {
                let p = p.unwrap();
                let neighbor_tile_row = &tiles[p.y];
                let neighbor_tile = neighbor_tile_row[p.x];
                neighbor_tile.tile_type != TileType::Empty
            }
        });
    starting_tile.resolve_starting_tile(starting_neighbors_nesw);
    tiles[starting_tile_point.y][starting_tile_point.x] = starting_tile;

    println!("{:?}", tiles);
}
