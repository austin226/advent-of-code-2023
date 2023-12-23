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
    points: Vec<Point>,
}

impl Brick {
    fn new(line: &str, id: usize) -> Self {
        static MODULE_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap());
        let caps = MODULE_REGEX.captures(line).unwrap();
        let nums =
            [1, 2, 3, 4, 5, 6].map(|i| caps.get(i).unwrap().as_str().parse::<i32>().unwrap());
        let (end1, end2) = (
            Point::new(nums[0], nums[1], nums[2]),
            Point::new(nums[3], nums[4], nums[5]),
        );
        Self {
            id,
            points: Self::points(end1, end2),
        }
    }

    fn points(end1: Point, end2: Point) -> Vec<Point> {
        let (xa, xb) = min_max(end1.x, end2.x);
        let (ya, yb) = min_max(end1.y, end2.y);
        let (za, zb) = min_max(end1.z, end2.z);

        let mut p = Vec::new();
        for x in xa..=xb {
            for y in ya..=yb {
                for z in za..=zb {
                    p.push(Point::new(x, y, z));
                }
            }
        }

        // Sort points by z value
        p.sort_by(|p1, p2| p1.z.cmp(&p2.z));
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
            // println!("Place brick {} at {:?}", brick.id, brick.points);
            for point in brick.points.iter() {
                max_x = i32::max(max_x, point.x);
                max_y = i32::max(max_y, point.y);
                max_z = i32::max(max_z, point.z);

                cells.insert(*point, brick.id);
                // println!("Brick {} is at {:?}", brick.id, point);
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
                        // println!("Found brick {} at {:?}", brick, point);
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
        let brick_id = match self.cells.get(point) {
            Some(b) => *b,
            None => return,
        };
        if self.collapsed_bricks.contains(&brick_id) {
            return;
        }
        let brick = self.bricks.get(brick_id).unwrap();
        let old_brick_points = brick.points.clone();

        // Find distance to fall
        // First point is guaranteed lowest z
        let min_z = old_brick_points[0].z;
        let highest_z_below = old_brick_points
            .iter()
            .filter(|p| p.z == min_z)
            .map(|p| self.highest_occupied_z_below(p))
            .max()
            .unwrap();
        let fall_distance = min_z - highest_z_below - 1;

        // Move points down by fall_distance
        let new_brick_points = old_brick_points
            .iter()
            .map(|old_point| Point {
                x: old_point.x,
                y: old_point.y,
                z: old_point.z - fall_distance,
            })
            .collect_vec();

        // Mark brick as already collapsed
        self.collapsed_bricks.insert(brick_id);

        // Update cells
        for old_point in old_brick_points {
            self.cells.remove(&old_point);
        }
        for new_point in new_brick_points.iter() {
            self.cells.insert(*new_point, brick_id);
        }

        // Update brick points
        let brick = self.bricks.get_mut(brick_id).unwrap();
        // println!("Moved brick {} to {:?}", brick_id, new_brick_points);
        brick.points = new_brick_points;
    }

    /// Return a map of brick IDs to brick IDs that support it.
    fn calculate_supported_bricks(&self) -> HashMap<usize, HashSet<usize>> {
        let mut res = HashMap::new();
        for brick in self.bricks.iter() {
            let mut neighbor_ids = HashSet::new();
            let max_z = brick
                .points
                .iter()
                .max_by(|p1, p2| p1.z.cmp(&p2.z))
                .unwrap()
                .z;
            let top_points = brick.points.iter().filter(|p| p.z == max_z);
            for point in top_points {
                let neighbor_point_down = Point::new(point.x, point.y, point.z + 1);
                if let Some(&neighbor_id) = self.cells.get(&neighbor_point_down) {
                    if neighbor_id != brick.id {
                        neighbor_ids.insert(neighbor_id);
                    }
                }
            }
            res.insert(brick.id, neighbor_ids);
        }
        res
    }

    /// Return a map of brick IDs to brick IDs that support it.
    fn calculate_base_bricks(&self) -> HashMap<usize, HashSet<usize>> {
        let mut res = HashMap::new();
        for brick in self.bricks.iter() {
            let mut neighbor_ids = HashSet::new();
            let min_z = brick
                .points
                .iter()
                .min_by(|p1, p2| p1.z.cmp(&p2.z))
                .unwrap()
                .z;
            let bot_points = brick.points.iter().filter(|p| p.z == min_z);
            for point in bot_points {
                let neighbor_point_down = Point::new(point.x, point.y, point.z - 1);
                if let Some(&neighbor_id) = self.cells.get(&neighbor_point_down) {
                    if neighbor_id != brick.id {
                        neighbor_ids.insert(neighbor_id);
                    }
                }
            }
            res.insert(brick.id, neighbor_ids);
        }
        res
    }

    fn highest_occupied_z_below(&self, start_point: &Point) -> i32 {
        for z in (1..start_point.z).rev() {
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

    fn count_chain_reactions(&self) -> i32 {
        let supported_bricks = self.calculate_supported_bricks();
        // println!("Supported bricks: {:?}", supported_bricks);
        let base_bricks = self.calculate_base_bricks();
        // println!("Supporting bricks: {:?}", base_bricks);

        let mut res = 0;
        let removed_bricks = HashSet::new();
        for brick in self.bricks.iter() {
            // println!("Simulate remove {}", brick.id);
            let n = Self::simulate_remove_brick(
                brick.id,
                &supported_bricks,
                &base_bricks,
                &removed_bricks,
            );
            // println!("{} bricks fall if {} is removed", n.len(), brick.id);
            res += n.len();
        }
        res as i32
    }

    /// Return the number of bricks that would fall if brick_id is removed
    fn simulate_remove_brick(
        brick_id: usize,
        supported_bricks: &HashMap<usize, HashSet<usize>>,
        base_bricks: &HashMap<usize, HashSet<usize>>,
        removed_bricks: &HashSet<usize>,
    ) -> HashSet<usize> {
        let mut output = HashSet::new();

        let mut rem = removed_bricks.clone();
        rem.insert(brick_id);

        // Figure out which bricks this one was supporting
        let formerly_supported_bricks = &supported_bricks[&brick_id];

        // If any of those are no longer supported, add them to the output
        for supported in formerly_supported_bricks.iter() {
            // Find which bricks should be supporting this one
            let mut remaining_base = base_bricks[supported].clone();

            // Remove everything in rem
            for removed_brick in rem.iter() {
                remaining_base.remove(removed_brick);
            }

            // Now if it's no longer supported, add it to the output
            if remaining_base.is_empty() {
                output.insert(*supported);

                // This brick was removed as well, so simulate it
                rem.insert(*supported);
                let chain_output =
                    Self::simulate_remove_brick(*supported, supported_bricks, base_bricks, &rem);
                for item in chain_output.iter() {
                    output.insert(*item);
                }
            }
        }

        output
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
    let mut tower = Tower::new(bricks);
    tower.collapse();
    // println!("{:?}", tower.cells.len());

    let ans = tower.count_chain_reactions();
    println!("{}", ans);
}
