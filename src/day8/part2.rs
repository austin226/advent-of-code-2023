use std::{collections::HashMap, time::Duration};

use crate::common::get_input;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
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
    let input = get_input("src/day8/input2.txt");

    let directions = parse_directions(&input[0]);
    let nodes = parse_nodes(&input);
    // println!("{:?}", nodes);

    // Start at all nodes ending in A
    let mut current_nodes = nodes
        .keys()
        .filter(|n| n.ends_with("A"))
        .map(|n| n.clone())
        .collect_vec();
    let mut dir_idx = 0;
    let mut n_steps = 0;
    println!("Started at {:?}", current_nodes);

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&["_..", "._.", ".._", "..."]),
    );
    println!("Stepping...");

    // While not all current nodes end in z
    while !current_nodes.iter().all(|n| n.ends_with("Z")) {
        // Count each step.
        n_steps += 1;
        pb.set_message(format!("{}", n_steps));

        let direction = &directions[dir_idx];

        current_nodes = current_nodes
            .iter()
            .map(|n| nodes[n].next(direction))
            .collect_vec();
        // println!("{:?} to {:?}", direction, current_nodes);

        // Go to next direction, or wrap back around.
        dir_idx = if dir_idx == directions.len() - 1 {
            0
        } else {
            dir_idx + 1
        };
    }
    pb.finish_with_message("Done");

    println!("{n_steps} steps.");
}

fn parse_directions(line: &String) -> Vec<Direction> {
    line.chars().map(|c| Direction::parse(c)).collect()
}

fn parse_nodes(input: &Vec<String>) -> HashMap<String, Node> {
    let mut nodes = HashMap::new();
    for line in &input[2..] {
        let re = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();
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
