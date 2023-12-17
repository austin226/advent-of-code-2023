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
struct Node {
    row: usize,
    col: usize,
    heat_loss: u8,
    variants: [NodeVariant; 12],
}

impl Node {
    fn new(row: usize, col: usize, heat_loss: u8) -> Self {
        use NodeVariant::*;
        let variants = [U1, U2, U3, R1, R2, R3, D1, D2, D3, L1, L2, L3];
        Self { row, col, heat_loss, variants }
    }
}

#[derive(Debug)]
struct Graph {
    size: usize,
    matrix: Vec<Vec<Node>>,
}

impl Graph {
    fn new(input: &Vec<String>) -> Self {
        let size = input.len();
        let matrix: Vec<Vec<Node>> = input.iter().enumerate().map(|(row, row_str)| {
            assert_eq!(size, row_str.len(), "Matrix must be square");
            row_str.chars().enumerate().map(|(col, c)| {
                let heat_loss = c.to_digit(10).expect("Parsing a digit") as u8;
                Node::new(row, col, heat_loss)
            }).collect_vec()
        }).collect_vec();
        Self { size, matrix }
    }

    fn next_position(&self, start: Position, direction: Direction) -> Option<Position> {
        let (next_row, next_col) = self.next_point(start.row, start.col, direction)?;
        let next_variant = self.next_variant(start.variant, direction)?;
        Some(Position { row: next_row, col: next_col, variant: next_variant })
    }

    fn node_at(&self, row: usize, col: usize) -> Option<&Node> {
        self.matrix.get(row)?.get(col)
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

    fn heuristic(&self, start: (usize, usize), goal: (usize, usize)) -> u64 {
        let max_row = std::cmp::max(start.0, goal.0);
        let min_row = std::cmp::min(start.0, goal.0);
        let max_col = std::cmp::max(start.1, goal.1);
        let min_col = std::cmp::min(start.1, goal.1);
        ((max_row - min_row) + (max_col - min_col)) as u64
    }

    fn get_heat(&self, position: &Position) -> u64 {
        self.node_at(position.row, position.col).unwrap().heat_loss as u64
    }

    fn reconstruct_path(&self, came_from: &HashMap<Position, Position>, current: &Position) -> u64 {
        let mut current = current;
        let mut total = self.get_heat(current);
        while came_from.contains_key(current) {
            println!("{:?}-{:?}", current, self.get_heat(current));
            current = &came_from[current];
            total += self.get_heat(current);
        }
        // Don't include first item
        total - self.node_at(current.row, current.col).unwrap().heat_loss as u64
    }

    fn a_star(&self, start_pos: &Position, goal: (usize, usize)) -> u64 {
        let mut open_pq = PriorityQueue::<Position, Reverse<u64>>::new();
        open_pq.push(*start_pos, Reverse(0));

        let mut came_from = HashMap::<Position, Position>::new();

        let mut g_score = HashMap::new();
        g_score.insert(*start_pos, 0u64);

        while !open_pq.is_empty() {
            let (current, _) = open_pq.pop().expect("Pop");
            println!("current={:?}", current);
            if current.row == goal.0 && current.col == goal.1 {
                // Found path to goal
                return self.reconstruct_path(&came_from, &current);
            }

            for neighbor in self.get_neighbors(&current) {
                let edge_weight = self.node_at(neighbor.row, neighbor.col).unwrap_or_else(|| panic!("Node ({},{}) not found", neighbor.row, neighbor.col)).heat_loss;
                let tentative_g_score = g_score.get(&current);
                if let Some(&tentative_g_score) = tentative_g_score {
                    let tentative_g_score = tentative_g_score + edge_weight as u64;
                    let neighbor_g_score = g_score.get(&neighbor);
                    if neighbor_g_score.is_none() || tentative_g_score < *neighbor_g_score.unwrap() {
                        // This is the best path to neighbor
                        came_from.insert(neighbor, current);
                        g_score.insert(neighbor, tentative_g_score);

                        let neighbor_f_score = tentative_g_score + self.heuristic((neighbor.row, neighbor.col), goal);
                        open_pq.push(neighbor, Reverse(neighbor_f_score));
                    }
                }
            }
        }
        panic!("Failed to find a path");
    }
}

pub fn run() {
    let input = get_input("src/day17/input0.txt");
    let graph = Box::new(Graph::new(&input));
    let start_pos = Position {
        row: 0,
        col: 0,
        variant: NodeVariant::U1,
    };
    let ans = graph.a_star(&start_pos, (graph.size - 1, graph.size - 1));
    println!("{ans}");
}
