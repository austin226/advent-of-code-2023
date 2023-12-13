use std::collections::HashMap;

use crate::common::get_input;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
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

fn nodes_ending_in(s: char, nodes: &HashMap<String, Node>) -> Vec<String> {
    nodes
        .keys()
        .filter(|n| n.ends_with(s))
        .map(|n| n.clone())
        .collect_vec()
}

fn get_node<'a>(nodes: &'a HashMap<String, Node>, name: &String) -> &'a Node {
    &nodes[name]
}

pub fn run() {
    let input = get_input("src/day8/input2.txt");

    let directions = parse_directions(&input[0]);
    let nodes = parse_nodes(&input);

    let a_nodes = nodes_ending_in('A', &nodes);

    // Find all loops
    let mut loop_multiples = Vec::new();
    loop_multiples.reserve(a_nodes.len());
    for a_node in a_nodes.iter() {
        let mut n_steps = 0;
        let mut current_node = a_node.to_owned();
        let mut i = 0;
        // println!("Start at {:?}", current_node);

        while !current_node.ends_with("Z") {
            // Continue
            let next_dir = &directions[i];
            let next_node = get_node(&nodes, &current_node).next(&next_dir);
            assert_ne!(current_node, next_node);
            // println!("From {:?}, {:?} to {:?}", current_node, next_dir, next_node);
            current_node = next_node;

            i += 1;
            if i == directions.len() {
                i = 0;
            }
            n_steps += 1;
        }

        loop_multiples.push(n_steps as i64);
    }
    println!("{:?}", loop_multiples);

    loop_multiples.sort();
    let mut lcm = loop_multiples[0];
    for m in loop_multiples {
        lcm = num::integer::lcm(lcm, m);
    }
    println!("{:?}", lcm);

    // println!("{n_steps} steps.");
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
