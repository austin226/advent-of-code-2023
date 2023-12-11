use queues::*;
use std::collections::{HashMap, HashSet};

use crate::common::get_input;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Tile {
    tile_type: TileType,
    point: Point,
    in_loop: bool,
}

impl Tile {
    fn neighbor_pipe_points(&self, map_width: usize, map_height: usize) -> Vec<Point> {
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

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
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

    fn is_empty(&self) -> bool {
        *self == TileType::Empty
    }

    fn to_block_segment(&self, r: usize, c: usize) -> bool {
        use TileType::*;
        match self {
            NS => match (r, c) {
                (0, 1) => true,
                (1, 1) => true,
                (2, 1) => true,
                _ => false,
            },
            EW => match (r, c) {
                (1, 0) => true,
                (1, 1) => true,
                (1, 2) => true,
                _ => false,
            },
            NE => match (r, c) {
                (0, 1) => true,
                (1, 1) => true,
                (1, 2) => true,
                _ => false,
            },
            NW => match (r, c) {
                (0, 1) => true,
                (1, 1) => true,
                (1, 0) => true,
                _ => false,
            },
            SW => match (r, c) {
                (1, 0) => true,
                (1, 1) => true,
                (2, 1) => true,
                _ => false,
            },
            SE => match (r, c) {
                (1, 1) => true,
                (1, 2) => true,
                (2, 1) => true,
                _ => false,
            },
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn neighbor_points(&self, map_width: usize, map_height: usize) -> Vec<Point> {
        let dxs = vec![0, 1, 0, -1];
        let dys = vec![-1, 0, 1, 0];
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
        let x = self.x as i32 + dx;
        let y = self.y as i32 + dy;
        if (0..map_width as i32).contains(&x) && (0..map_height as i32).contains(&y) {
            return Some(Point {
                x: x as usize,
                y: y as usize,
            });
        }
        return None;
    }
}

#[derive(Debug, Clone)]
struct BigGridTile {
    big_grid_point: Point,
    orig_point: Point,
    is_loop: bool,
    is_outside: bool,
}

impl BigGridTile {
    fn new() -> Self {
        Self {
            big_grid_point: Point { x: 0, y: 0 },
            orig_point: Point { x: 0, y: 0 },
            is_loop: false,
            is_outside: false,
        }
    }
}

fn get_tile(tiles: &Vec<Vec<Tile>>, point: &Point) -> Tile {
    tiles[point.y][point.x]
}

fn get_tile_mut<'a>(tiles: &'a mut Vec<Vec<Tile>>, point: &Point) -> &'a mut Tile {
    &mut tiles[point.y][point.x]
}

fn get_big_tile<'a>(
    big_grid: &'a Vec<Vec<BigGridTile>>,
    big_grid_point: &Point,
) -> &'a BigGridTile {
    &big_grid[big_grid_point.y][big_grid_point.x]
}

fn get_big_tile_mut<'a>(
    big_grid: &'a mut Vec<Vec<BigGridTile>>,
    big_grid_point: &Point,
) -> &'a mut BigGridTile {
    &mut big_grid[big_grid_point.y][big_grid_point.x]
}

pub fn run() {
    // Input is a square of pipe symbols
    let input = get_input("src/day10/input7.txt");

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
                in_loop: false,
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
        let starting_tile = get_tile(&tiles, &starting_tile_point);
        let starting_neighbors_nesw = [(0, -1), (1, 0), (0, 1), (-1, 0)]
            .map(|(dx, dy)| starting_tile.point_in_dir(dy, dx, map_width, map_height))
            .map(|p| match p {
                Some(p) => get_tile(&tiles, &p).tile_type,
                None => TileType::Empty,
            });
        get_tile_mut(&mut tiles, &starting_tile_point)
            .resolve_starting_tile(starting_neighbors_nesw);
    }

    // Do a BFS to mark loop tiles
    {
        let mut visited = HashSet::<Point>::new();

        // queue point and distance from start
        let mut q: Queue<Point> = queue![];
        let _ = q.add(starting_tile_point);
        while q.size() > 0 {
            let v_p = q.remove().unwrap();
            visited.insert(v_p);
            let v = get_tile_mut(&mut tiles, &v_p);
            v.in_loop = true;
            for u_p in v.neighbor_pipe_points(map_width, map_height) {
                let u = get_tile(&tiles, &u_p);
                if !u.tile_type.is_empty() {
                    // u is a pipe. Let's visit it if not yet visited.
                    if !visited.contains(&u_p) {
                        let _ = q.add(u_p);
                    }
                }
            }
        }
    }

    // make a big grid
    const BLOCK_SIZE: usize = 3;
    let mut big_grid: Vec<Vec<BigGridTile>> =
        vec![vec![BigGridTile::new().clone(); map_width * BLOCK_SIZE]; map_height * BLOCK_SIZE];
    for src_y in 0..map_height {
        let block_start_y = src_y * BLOCK_SIZE;
        for src_x in 0..map_width {
            let block_start_x = src_x * BLOCK_SIZE;
            let orig_point = Point { x: src_x, y: src_y };
            let tile = get_tile(&tiles, &orig_point);
            for block_segment_r in 0..BLOCK_SIZE {
                let dst_y = block_start_y + block_segment_r;
                for block_segment_c in 0..BLOCK_SIZE {
                    let dst_x = block_start_x + block_segment_c;
                    let block_tile_in_loop = tile.in_loop
                        && tile
                            .tile_type
                            .to_block_segment(block_segment_r, block_segment_c);
                    big_grid[dst_y][dst_x].big_grid_point = Point { x: dst_x, y: dst_y };
                    big_grid[dst_y][dst_x].orig_point = orig_point;
                    big_grid[dst_y][dst_x].is_loop = block_tile_in_loop;
                }
            }
        }
    }

    // TODO ideally we'd draw a border around the big_grid to guarantee there's no loop pieces around the outside

    // BFS
    // flood fill from 0,0
    {
        let start_p = Point { x: 0, y: 0 };
        get_big_tile_mut(&mut big_grid, &start_p).is_outside = true;

        let mut q: Queue<Point> = queue![];
        let _ = q.add(start_p);
        while q.size() > 0 {
            let v_p = q.remove().unwrap();
            for u_p in v_p.neighbor_points(map_width * BLOCK_SIZE, map_height * BLOCK_SIZE) {
                let u = get_big_tile_mut(&mut big_grid, &u_p);
                if !u.is_loop && !u.is_outside {
                    u.is_outside = true;
                    println!("q {:?}", u_p);
                    let _ = q.add(u_p);
                }
            }
        }
    }

    // Collect all big grid tiles not marked as loop or outside
    let mut inside_tiles = HashSet::new();
    for r in big_grid.iter() {
        for t in r {
            if !t.is_loop && !t.is_outside {
                // t is inside loop
                // Check if the original point was part of the loop
                let orig_tile = get_tile(&tiles, &t.orig_point);
                if !orig_tile.in_loop {
                    inside_tiles.insert(orig_tile);
                }
            }
        }
    }

    for r in big_grid.iter() {
        for c in r {
            if c.is_loop {
                print!("8");
            } else if c.is_outside {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    println!("{}", inside_tiles.len());

    // println!("{:?}", tiles);
}
