use std::ops::Index;

use itertools::Itertools;

use crate::common::get_input;
use crate::day17::part1::Direction::{Down, Left, Right, Up};
use crate::day17::part1::NodeVariant::{D1, D2, L1, L2, R1, R2, R3, U1, U2, U3};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
struct Position<'n> {
    node: &'n Node,
    variant: NodeVariant,
}

#[derive(Debug, Copy, Clone)]
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

    fn next_node(&self, start: Position, direction: Direction) -> Option<Position> {
        let (next_row, next_col) = self.next_point(start.node.row, start.node.col, direction)?;
        let next_node = self.node_at(next_row, next_col)?;
        let next_variant = self.next_variant(start.variant, direction)?;
        Some(Position { node: next_node, variant: next_variant })
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
            (_, Up) => Some(U1),

            // Right
            (R1, Right) => Some(R2),
            (R2, Right) => Some(R3),
            (R3, Right) => None,
            (_, Right) => Some(R1),

            // Down
            (D1, Down) => Some(D2),
            (D2, Down) => Some(D3),
            (D3, Down) => None,
            (_, Down) => Some(D1),

            // Left
            (L1, Left) => Some(L2),
            (L2, Left) => Some(L3),
            (L3, Left) => None,
            (_, Left) => Some(L1),
        }
    }
}

pub fn run() {
    let input = get_input("src/day17/input0.txt");
    let graph = Box::new(Graph::new(&input));
    let start_pos = Position {
        node: graph.node_at(0, 0).expect("starting node"),
        variant: NodeVariant::U1,
    };
    // println!("{:?}", graph);
}
