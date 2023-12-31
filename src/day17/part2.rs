use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::Formatter;

use itertools::Itertools;
use priority_queue::PriorityQueue;

use crate::common::get_input;
use crate::day17::part2::Direction::{Down, Left, Right, Up};
use crate::day17::part2::NodeVariant::{D, Init, L, R, U};

const MIN_DIST_BEFORE_TURN: u8 = 4;
const MAX_STRAIGHT_DIST: u8 = 10;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
    variant: NodeVariant,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{:?})", self.row, self.col, self.variant)
    }
}

/// Holds historical data on how far we've traveled in a particular direction.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum NodeVariant {
    Init,
    U(u8),
    R(u8),
    D(u8),
    L(u8),
}

impl NodeVariant {
    fn can_turn(&self) -> bool {
        use NodeVariant::*;
        match self {
            Init => true,
            U(dist) | R(dist) | D(dist) | L(dist) => {
                *dist >= MIN_DIST_BEFORE_TURN
            }
        }
    }

    fn can_continue_straight(&self) -> bool {
        use NodeVariant::*;
        match self {
            Init => false,
            U(dist) | R(dist) | D(dist) | L(dist) => {
                *dist < MAX_STRAIGHT_DIST
            }
        }
    }

    fn turn(direction: Direction) -> Self {
        match direction {
            Up => U(1),
            Right => R(1),
            Down => D(1),
            Left => L(1)
        }
    }

    fn try_continue_straight(&self) -> Option<Self> {
        if !self.can_continue_straight() {
            return None;
        }

        use NodeVariant::*;
        Some(match self {
            Init => panic!("Init cannot continue straight"),
            U(dist) => U(dist + 1),
            R(dist) => R(dist + 1),
            D(dist) => D(dist + 1),
            L(dist) => L(dist + 1),
        })
    }

    fn try_move(&self, direction: Direction) -> Option<Self> {
        use NodeVariant::*;
        use Direction::*;
        match (self, direction) {
            // Init
            (Init, direction) => Some(Self::turn(direction)),

            // 180 degrees - never allowed
            (U(_), Down) | (R(_), Left) | (D(_), Up) | (L(_), Right) => None,

            // Continuing straight
            (U(dist), Up) | (R(dist), Right) | (D(dist), Down) | (L(dist), Left) => self.try_continue_straight(),

            // 90 degrees
            _ => self.try_turn(direction)
        }
    }

    fn try_turn(&self, direction: Direction) -> Option<Self> {
        if self.can_turn() {
            Some(Self::turn(direction))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Graph {
    height: usize,
    width: usize,
    matrix: Vec<Vec<u64>>,
}

impl Graph {
    fn new(input: &Vec<String>) -> Self {
        let height = input.len();
        let width = input[0].len();
        let matrix: Vec<Vec<u64>> = input.iter().enumerate().map(|(row, row_str)| {
            assert_eq!(width, row_str.len(), "Matrix must be rectangular");
            row_str.chars().enumerate().map(|(col, c)| {
                c.to_digit(10).expect("Parsing a digit") as u64
            }).collect_vec()
        }).collect_vec();
        Self { height, width, matrix }
    }

    fn next_position(&self, start: Position, direction: Direction) -> Option<Position> {
        let (next_row, next_col) = self.next_point(start.row, start.col, direction)?;
        let next_variant = self.next_variant(start.variant, direction)?;
        Some(Position { row: next_row, col: next_col, variant: next_variant })
    }

    fn heat_loss_at(&self, position: &Position) -> u64 {
        self.matrix[position.row][position.col]
    }

    fn next_point(&self, row: usize, col: usize, direction: Direction) -> Option<(usize, usize)> {
        use Direction::*;
        match direction {
            Up => {
                if row == 0 {
                    None
                } else {
                    Some((row - 1, col))
                }
            }
            Right => {
                if col == self.width - 1 {
                    None
                } else {
                    Some((row, col + 1))
                }
            }
            Down => {
                if row == self.height - 1 {
                    None
                } else {
                    Some((row + 1, col))
                }
            }
            Left => {
                if col == 0 {
                    None
                } else {
                    Some((row, col - 1))
                }
            }
        }
    }

    fn next_variant(&self, start_variant: NodeVariant, direction: Direction) -> Option<NodeVariant> {
        start_variant.try_move(direction)
    }

    fn get_neighbors(&self, start_position: &Position) -> Vec<Position> {
        let mut neighbors = Vec::new();

        use Direction::*;
        for direction in [Up, Right, Down, Left] {
            if let Some((next_row, next_col)) = self.next_point(start_position.row, start_position.col, direction) {
                if let Some(next_variant) = self.next_variant(start_position.variant, direction) {
                    neighbors.push(Position { row: next_row, col: next_col, variant: next_variant });
                }
            }
        }
        // println!("from {:?}, neighbors are {:?}", start_position, neighbors);
        neighbors
    }

    fn heuristic(&self, start_pos: &Position) -> u64 {
        // Assume goal is to bottom-right of, or equal to Position
        let goal = self.goal();
        ((goal.0 - start_pos.row) + (goal.1 - start_pos.col)) as u64
    }

    fn reconstruct_path(&self, came_from: &HashMap<Position, Position>, current: &Position) -> u64 {
        let mut current = current;
        let mut total = self.heat_loss_at(current);
        while came_from.contains_key(current) {
            // println!("{:?}-{:?}", current, self.heat_loss_at(current));
            current = &came_from[current];
            total += self.heat_loss_at(current);
        }
        // Don't include first item
        total - self.heat_loss_at(current)
    }

    fn a_star(&self, start_pos: &Position) -> u64 {
        let mut open_pq = PriorityQueue::<Position, Reverse<u64>>::new();
        open_pq.push(*start_pos, Reverse(0));

        let mut came_from = HashMap::<Position, Position>::new();

        let mut g_score = HashMap::new();
        g_score.insert(*start_pos, 0u64);

        while !open_pq.is_empty() {
            let (current, _) = open_pq.pop().expect("Pop");
            // println!("current={:?}", current);
            if (current.row, current.col) == self.goal() {
                // Found path to goal
                // But path must be able to turn in order to stop
                if current.variant.can_turn() {
                    return self.reconstruct_path(&came_from, &current);
                }
            }

            for neighbor in self.get_neighbors(&current) {
                if let Some(&tentative_g_score) = g_score.get(&current) {
                    let tentative_g_score = tentative_g_score + self.heat_loss_at(&neighbor);
                    if g_score.get(&neighbor).map_or(true, |g| tentative_g_score < *g) {
                        // This is the best path to neighbor
                        came_from.insert(neighbor, current);
                        g_score.insert(neighbor, tentative_g_score);
                        let h = self.heuristic(&neighbor);
                        let neighbor_f_score = tentative_g_score + h;
                        open_pq.push(neighbor, Reverse(neighbor_f_score));
                    }
                }
            }
        }
        panic!("Failed to find a path");
    }

    fn goal(&self) -> (usize, usize) {
        (self.height - 1, self.width - 1)
    }
}

pub fn run() {
    let input = get_input("src/day17/input1.txt");
    let graph = Box::new(Graph::new(&input));
    let start_pos = Position {
        row: 0,
        col: 0,
        variant: Init,
    };
    let ans = graph.a_star(&start_pos);
    println!("{ans}");
}
