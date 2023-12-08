use std::collections::HashMap;

use crate::common::get_input;
use regex::Regex;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    fn next(&self, direction: &Direction) -> String {
        match direction {
            Direction::Left => self.left.clone(),
            Direction::Right => self.right.clone(),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse(c: char) -> Self {
        match c {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => panic!("Invalid direction: {c}"),
        }
    }
}

pub fn run() {
    let input = get_input("src/day8/input1.txt");

    let directions = parse_directions(&input[0]);
    let nodes = parse_nodes(&input);

    // Start at node AAA
    let mut current_node = "AAA".to_string();
    let mut dir_idx = 0;
    let mut n_steps = 0;
    println!("Started at {current_node}");
    while current_node != "ZZZ" {
        // Count each step.
        n_steps += 1;

        let direction = &directions[dir_idx];
        current_node = nodes[&current_node].next(direction);
        println!("{:?} to {current_node}", direction);

        // Go to next direction, or wrap back around.
        dir_idx = if dir_idx == directions.len() - 1 {
            0
        } else {
            dir_idx + 1
        };
    }

    println!("{n_steps} steps.");
}

fn parse_directions(line: &String) -> Vec<Direction> {
    line.chars().map(|c| Direction::parse(c)).collect()
}

fn parse_nodes(input: &Vec<String>) -> HashMap<String, Node> {
    let mut nodes = HashMap::new();
    for line in &input[2..] {
        let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
        for (_, [n, l, r]) in re.captures_iter(line).map(|caps| caps.extract()) {
            let name = n.to_string();
            let left = l.to_string();
            let right = r.to_string();
            let node = Node { left, right };
            nodes.insert(name, node);
        }
    }
    return nodes;
}
