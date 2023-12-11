use colored::Colorize;
use core::fmt;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use itertools::Itertools;
use queues::*;
use std::io;
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};
use tui::{backend::CrosstermBackend, Terminal};

use crate::common::get_input;

#[derive(Copy, Clone, Debug)]
struct Tile {
    tile_type: TileType,
    point: Point,
    is_loop: bool,
    is_occupied: bool,
    is_outside: bool,
    is_inside: bool,
    is_current_left: bool,
    is_current_facing: bool,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_occupied {
            write!(f, "{}", "☺")
        } else if self.is_current_facing {
            write!(f, "{}", format!("{}", self.tile_type).bright_yellow())
        } else if self.is_current_left {
            write!(f, "{}", format!("{}", self.tile_type).red())
        } else if self.is_inside {
            write!(f, "{}", format!("{}", self.tile_type).purple())
        } else if self.is_outside {
            write!(f, "{}", format!("{}", self.tile_type).blue())
        } else if self.is_loop {
            write!(f, "{}", format!("{}", self.tile_type).green())
        } else {
            write!(f, "{}", format!("{}", self.tile_type).yellow())
        }
    }
}

impl Tile {
    fn new(tile_type: TileType, x: usize, y: usize) -> Self {
        Self {
            tile_type,
            point: Point { x, y },
            is_loop: false,
            is_occupied: false,
            is_outside: false,
            is_inside: false,
            is_current_left: false,
            is_current_facing: false,
        }
    }

    fn is_inside(&self) -> bool {
        !self.is_loop && !self.is_outside
    }

    fn neighbor_points(&self, map_width: usize, map_height: usize) -> Vec<Point> {
        let dys = [-1, 0, 1, 0];
        let dxs = [0, 1, 0, -1];
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
            NE => "└",
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

fn get_tile<'a>(tiles: &'a Vec<Vec<Tile>>, point: &Point) -> &'a Tile {
    &tiles[point.y][point.x]
}

fn get_tile_mut<'a>(tiles: &'a mut Vec<Vec<Tile>>, point: &Point) -> &'a mut Tile {
    &mut tiles[point.y][point.x]
}

/// Returns the point where the next tile in the loop is located.
fn next_loop_tile(
    tiles: &Vec<Vec<Tile>>,
    point: &Point,
    prev_point: &Option<Point>,
    map_width: usize,
    map_height: usize,
) -> Point {
    let current_tile = get_tile(tiles, point);
    assert!(current_tile.is_loop, "Tile {:?} not in loop!", point);

    let neighbor_points = current_tile.neighbor_pipe_points(map_width, map_height);
    let neighbors_in_loop = neighbor_points
        .iter()
        .filter(|n| get_tile(tiles, n).is_loop)
        .collect_vec();
    assert_eq!(2, neighbors_in_loop.len(), "Should be 2 neighbors in loop");

    if let Some(prev_point) = prev_point {
        if prev_point == neighbors_in_loop[0] {
            return *neighbors_in_loop[1];
        }
        return *neighbors_in_loop[0];
    }

    // TODO
    // Determine clockwise direction
    return *neighbors_in_loop[0];
}

fn print_grid<W>(tiles: &Vec<Vec<Tile>>, terminal: &mut Terminal<CrosstermBackend<W>>)
where
    W: std::io::Write,
{
    let _ = terminal.clear();
    for row in tiles.iter() {
        for tile in row.iter() {
            print!("{}", tile);
        }
        println!();
    }
}

fn mark_tiles_outside(
    tiles: &mut Vec<Vec<Tile>>,
    outside_tile_point: &Point,
    map_width: usize,
    map_height: usize,
) {
    // Do a BFS
    let mut visited = HashSet::<Point>::new();

    // queue point
    let mut q: Queue<Point> = queue![];
    let _ = q.add(*outside_tile_point);
    let mut in_q = HashSet::<Point>::new();
    in_q.insert(*outside_tile_point);
    while q.size() > 0 {
        let v_p = q.remove().unwrap();
        get_tile_mut(tiles, &v_p).is_outside = true;
        visited.insert(v_p);
        let v = get_tile(&tiles, &v_p);
        for u_p in v.neighbor_points(map_width, map_height) {
            let u = get_tile(&tiles, &u_p);
            if !u.is_loop && !u.is_outside && !visited.contains(&u_p) && !in_q.contains(&u_p) {
                let _ = q.add(u_p);
                in_q.insert(u_p);
            }
        }
    }
}

pub fn run() {
    // Set up terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Input is a square of pipe symbols
    let input = get_input("src/day10/input_full.txt");

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
            let tile = Tile::new(tile_type, x, y);
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
        let starting_tile = get_tile_mut(&mut tiles, &starting_tile_point);
        starting_tile.resolve_starting_tile(starting_neighbors_nesw);
    }

    // Do a BFS to determine how far the furthest connected tile is from the start
    {
        let mut visited = HashSet::<Point>::new();

        // queue point and distance from start
        let mut q: Queue<(Point, u32)> = queue![];
        let mut distances = HashMap::<Point, u32>::new();
        let _ = q.add((starting_tile_point, 0));
        while q.size() > 0 {
            let (v_p, dist_from_start) = q.remove().unwrap();
            tiles[v_p.y][v_p.x].is_loop = true;
            if let Some(old_dist) = distances.get(&v_p) {
                distances.insert(v_p, std::cmp::min(*old_dist, dist_from_start));
            } else {
                distances.insert(v_p, dist_from_start);
            }
            // println!("Visit {:?}", v_p);
            visited.insert(v_p);
            let v = get_tile(&tiles, &v_p);
            for u_p in v.neighbor_pipe_points(map_width, map_height) {
                let u = get_tile(&tiles, &u_p);
                if !u.tile_type.is_empty() {
                    // u is a pipe. Let's visit it if not yet visited.
                    // println!("Neighbor of {:?} at {:?}", v_p, u_p);
                    // println!("Visited is {:?}", visited);
                    if !visited.contains(&u_p) {
                        // println!("Add {:?} to q", u_p);
                        // Mark the tile as part of the loop
                        let _ = q.add((u_p, dist_from_start + 1));
                    }
                }
            }
        }
        // println!("{:?}", distances.into_values().max().unwrap());
    }

    // Start at the starting point
    let mut current_point = starting_tile_point;
    let mut prev_point: Option<Point> = None;
    loop {
        // Walk the loop clockwise
        let next_point = next_loop_tile(&tiles, &current_point, &prev_point, map_width, map_height);
        if next_point == starting_tile_point {
            break;
        }

        // Starting at the left hand point, if it's not part of the loop, mark it and all connected points as outside.
        let facing_dir = (
            (next_point.y as i32 - current_point.y as i32),
            (next_point.x as i32 - current_point.x as i32),
        );
        let left_hand_dir = match facing_dir {
            // dy, dx
            (-1, 0) => (0, -1),
            (0, 1) => (-1, 0),
            (1, 0) => (0, 1),
            (0, -1) => (1, 0),
            _ => panic!("Bad facing_dir: {:?}", facing_dir),
        };
        let left_hand_point = get_tile(&tiles, &current_point).point_in_dir(
            left_hand_dir.0,
            left_hand_dir.1,
            map_width,
            map_height,
        );
        // println!(
        //     "Current: {:?}, next: {:?}, left: {:?}",
        //     current_point, next_point, left_hand_point
        // );
        if let Some(left_hand_point) = left_hand_point {
            let left_tile = get_tile_mut(&mut tiles, &left_hand_point);
            left_tile.is_current_left = true;
            if !left_tile.is_loop {
                mark_tiles_outside(&mut tiles, &left_hand_point, map_width, map_height);
            }
        }

        get_tile_mut(&mut tiles, &current_point).is_occupied = true;
        get_tile_mut(&mut tiles, &next_point).is_current_facing = true;

        // print_grid(&tiles, &mut terminal);
        // std::thread::sleep(Duration::from_millis(100));

        if let Some(left_hand_point) = left_hand_point {
            get_tile_mut(&mut tiles, &left_hand_point).is_current_left = false;
        }
        get_tile_mut(&mut tiles, &next_point).is_current_facing = false;
        get_tile_mut(&mut tiles, &current_point).is_occupied = false;

        // Update current location
        prev_point = Some(current_point);
        current_point = next_point;
    }

    // Count inside tiles
    let mut inside_count = 0;
    for y in 0..map_height {
        for x in 0..map_width {
            let tile = get_tile_mut(&mut tiles, &Point { x, y });
            if tile.is_inside() {
                tile.is_inside = true;
                inside_count += 1;
            }
        }
    }
    print_grid(&tiles, &mut terminal);

    println!("{:?}", inside_count);
}
