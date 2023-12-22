use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::common::get_input;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    id: usize,
    ends: (Point, Point),
}

impl Brick {
    fn new(line: &str, id: usize) -> Self {
        static MODULE_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap());
        let caps = MODULE_REGEX.captures(line).unwrap();
        let nums =
            [1, 2, 3, 4, 5, 6].map(|i| caps.get(i).unwrap().as_str().parse::<i32>().unwrap());
        Self {
            id,
            ends: (
                Point::new(nums[0], nums[1], nums[2]),
                Point::new(nums[3], nums[4], nums[5]),
            ),
        }
    }

    fn points(&self) -> Vec<Point> {
        let (xa, xb) = min_max(self.ends.0.x, self.ends.1.x);
        let (ya, yb) = min_max(self.ends.0.y, self.ends.1.y);
        let (za, zb) = min_max(self.ends.0.z, self.ends.1.z);

        let mut p = Vec::new();
        for x in xa..=xb {
            for y in ya..=yb {
                for z in za..=zb {
                    p.push(Point::new(x, y, z));
                }
            }
        }
        p
    }
}

#[derive(Debug)]
struct Tower {
    max_x: i32,
    max_y: i32,
    max_z: i32,
    cells: HashMap<Point, usize>,
    collapsed_bricks: HashSet<usize>,
    bricks: Vec<Brick>,
}

impl Tower {
    fn new(bricks: Vec<Brick>) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;
        let mut cells = HashMap::new();

        for brick in bricks.iter() {
            max_x = i32::max(max_x, brick.ends.0.x);
            max_x = i32::max(max_x, brick.ends.1.x);
            max_y = i32::max(max_y, brick.ends.0.y);
            max_y = i32::max(max_y, brick.ends.1.y);
            max_z = i32::max(max_z, brick.ends.0.z);
            max_z = i32::max(max_z, brick.ends.1.z);

            for point in brick.points() {
                cells.insert(point, brick.id);
            }
        }

        let collapsed_bricks = HashSet::new();

        Self {
            max_x,
            max_y,
            max_z,
            cells,
            collapsed_bricks,
            bricks,
        }
    }

    fn collapse(&mut self) {
        let mut points_with_bricks = Vec::new();
        for z in 2..=self.max_z {
            for x in 0..=self.max_x {
                for y in 0..=self.max_y {
                    let point = Point { x, y, z };
                    if let Some(brick) = self.cells.get(&point) {
                        points_with_bricks.push(point);
                    }
                }
            }
        }
        for point in points_with_bricks {
            self.collapse_brick(&point);
        }
    }

    fn collapse_brick(&mut self, point: &Point) {
        let brick_id = self.cells[point];
        if self.collapsed_bricks.contains(&brick_id) {
            return;
        }
        let brick = self.bricks.get_mut(brick_id).unwrap();
        let old_brick_points = brick.points();

        // TODO move

        // Mark brick as already collapsed
        self.collapsed_bricks.insert(brick_id);

        // Update cells
        for old_point in old_brick_points {
            self.cells.remove(&old_point);
        }
        let new_brick_points = brick.points();
        for new_point in new_brick_points {
            self.cells.insert(new_point, brick_id);
        }
    }

    fn highest_occupied_z_below(&self, start_point: &Point) -> i32 {
        for z in (1..=start_point.z).rev() {
            //
            let point = Point {
                x: start_point.x,
                y: start_point.y,
                z,
            };
            if self.cells.contains_key(&point) {
                return z;
            }
        }
        0
    }
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    let min = i32::min(a, b);
    let max = i32::max(a, b);
    (a, b)
}

pub fn run() {
    let input = get_input("src/day22/input0.txt");
    let bricks = input
        .iter()
        .enumerate()
        .map(|(id, line)| Brick::new(line.as_str(), id))
        .collect_vec();
    let tower = Tower::new(bricks);
    println!("{:?}", tower.cells.len());
}
