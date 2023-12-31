use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::ops::Index;

use itertools::Itertools;
use priority_queue::PriorityQueue;

use crate::common::get_input;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum NodeVariant {
    Init,
    U1,
    U2,
    U3,
    R1,
    R2,
    R3,
    D1,
    D2,
    D3,
    L1,
    L2,
    L3,
}

impl Index<NodeVariant> for [usize; 12] {
    type Output = usize;

    fn index(&self, node_variant: NodeVariant) -> &Self::Output {
        &self[node_variant as usize]
    }
}

#[derive(Debug)]
struct Graph {
    size: usize,
    matrix: Vec<Vec<u64>>,
}

impl Graph {
    fn new(input: &Vec<String>) -> Self {
        let size = input.len();
        let matrix: Vec<Vec<u64>> = input.iter().enumerate().map(|(row, row_str)| {
            assert_eq!(size, row_str.len(), "Matrix must be square");
            row_str.chars().enumerate().map(|(col, c)| {
                c.to_digit(10).expect("Parsing a digit") as u64
            }).collect_vec()
        }).collect_vec();
        Self { size, matrix }
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
                if col == self.size - 1 {
                    None
                } else {
                    Some((row, col + 1))
                }
            }
            Down => {
                if row == self.size - 1 {
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
        use NodeVariant::*;
        use Direction::*;
        match (start_variant, direction) {
            // Up
            (U1, Up) => Some(U2),
            (U2, Up) => Some(U3),
            (U3, Up) => None,
            (U1, Down) | (U2, Down) | (U3, Down) => None,

            // Right
            (R1, Right) => Some(R2),
            (R2, Right) => Some(R3),
            (R3, Right) => None,
            (R1, Left) | (R2, Left) | (R3, Left) => None,

            // Down
            (D1, Down) => Some(D2),
            (D2, Down) => Some(D3),
            (D3, Down) => None,
            (D1, Up) | (D2, Up) | (D3, Up) => None,

            // Left
            (L1, Left) => Some(L2),
            (L2, Left) => Some(L3),
            (L3, Left) => None,
            (L1, Right) | (L2, Right) | (L3, Right) => None,

            (_, Up) => Some(U1),
            (_, Right) => Some(R1),
            (_, Down) => Some(D1),
            (_, Left) => Some(L1),
        }
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
                return self.reconstruct_path(&came_from, &current);
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
        (self.size - 1, self.size - 1)
    }
}

pub fn run() {
    let input = get_input("src/day17/input1.txt");
    let graph = Box::new(Graph::new(&input));
    let start_pos = Position {
        row: 0,
        col: 0,
        variant: NodeVariant::Init,
    };
    let ans = graph.a_star(&start_pos);
    println!("{ans}");
}
