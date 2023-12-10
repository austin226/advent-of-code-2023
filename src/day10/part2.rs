use core::fmt;
use queues::*;
use std::collections::{HashMap, HashSet};

use crate::common::get_input;

#[derive(Copy, Clone, Debug)]
struct Tile {
    tile_type: TileType,
    point: Point,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tile_type)
    }
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

    fn resolve_starting_tile(&mut self, neighbors_nesw: [TileType; 4]) {
        if !self.tile_type.is_start() {
            panic!("resolve_starting_tile called on non-starting tile");
        }

        use TileType::*;
        let connected_n = match neighbors_nesw[0] {
            NS | SW | SE => true,
            _ => false,
        };
        let connected_e = match neighbors_nesw[1] {
            EW | NW | SW => true,
            _ => false,
        };
        let connected_s = match neighbors_nesw[2] {
            NS | NW | NE => true,
            _ => false,
        };
        let connected_w = match neighbors_nesw[3] {
            EW | NE | SE => true,
            _ => false,
        };

        let new_type = match (connected_n, connected_e, connected_s, connected_w) {
            (true, true, false, false) => NE,
            (true, false, true, false) => NS,
            (true, false, false, true) => NW,
            (false, true, true, false) => SE,
            (false, true, false, true) => EW,
            (false, false, true, true) => SW,
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

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TileType::*;
        let d = match self {
            NS => "│",
            NE => "┖",
            NW => "┘",
            EW => "─",
            SW => "┐",
            SE => "┌",
            Start => "🐿️",
            _ => "░",
        };
        write!(f, "{}", d)
    }
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

    fn is_empty(&self) -> bool {
        *self == TileType::Empty
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn get_tile(tiles: &Vec<Vec<Tile>>, point: &Point) -> Tile {
    tiles[point.y][point.x]
}

fn print_grid(tiles: &Vec<Vec<Tile>>) {
    for row in tiles.iter() {
        for tile in row.iter() {
            print!("{}", tile);
        }
        println!();
    }
}

pub fn run() {
    // Input is a square of pipe symbols
    let input = get_input("src/day10/input4.txt");

    let map_width = input[0].len();
    let map_height = input.len();

    // Parse tiles and get the starting point
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let mut starting_tile_point: Option<Point> = None;
    for y in 0..map_height {
        let row_str = &input[y];
        let mut row = Vec::new();
        for x in 0..map_width {
            let tile_type = TileType::parse(row_str.as_bytes()[x] as char);
            let tile = Tile {
                tile_type,
                point: (Point { x, y }),
            };
            if tile_type.is_start() {
                starting_tile_point = Some(tile.point);
            }
            row.push(tile);
        }
        tiles.push(row);
    }

    // Determine starting tile directions based on neighboring points
    let starting_tile_point = starting_tile_point.unwrap();
    {
        let mut starting_tile = get_tile(&tiles, &starting_tile_point);
        let starting_neighbors_nesw = [(0, -1), (1, 0), (0, 1), (-1, 0)]
            .map(|(dx, dy)| starting_tile.point_in_dir(dy, dx, map_width, map_height))
            .map(|p| match p {
                Some(p) => get_tile(&tiles, &p).tile_type,
                None => TileType::Empty,
            });
        starting_tile.resolve_starting_tile(starting_neighbors_nesw);
        tiles[starting_tile_point.y][starting_tile_point.x] = starting_tile;
    }

    // Make tiles immutable
    let tiles = tiles;

    // Do a BFS to determine how far the furthest connected tile is from the start
    {
        let mut visited = HashSet::<Point>::new();

        // queue point and distance from start
        let mut q: Queue<(Point, u32)> = queue![];
        let mut distances = HashMap::<Point, u32>::new();
        let _ = q.add((starting_tile_point, 0));
        while q.size() > 0 {
            let (v_p, dist_from_start) = q.remove().unwrap();
            if let Some(old_dist) = distances.get(&v_p) {
                distances.insert(v_p, std::cmp::min(*old_dist, dist_from_start));
            } else {
                distances.insert(v_p, dist_from_start);
            }
            // println!("Visit {:?}", v_p);
            visited.insert(v_p);
            let v = get_tile(&tiles, &v_p);
            for u_p in v.neighbor_points(map_width, map_height) {
                let u = get_tile(&tiles, &u_p);
                if !u.tile_type.is_empty() {
                    // u is a pipe. Let's visit it if not yet visited.
                    // println!("Neighbor of {:?} at {:?}", v_p, u_p);
                    // println!("Visited is {:?}", visited);
                    if !visited.contains(&u_p) {
                        // println!("Add {:?} to q", u_p);
                        let _ = q.add((u_p, dist_from_start + 1));
                    }
                }
            }
        }
        println!("{:?}", distances.into_values().max().unwrap());
    }

    // println!("{:?}", tiles);
    print_grid(&tiles);
}
