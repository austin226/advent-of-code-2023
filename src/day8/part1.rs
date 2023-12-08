use crate::common::get_input;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
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
    let input = get_input("src/day8/input0.txt");

    let directions = parse_directions(&input[0]);
    let nodes = parse_nodes(&input);

    println!("{:?}", directions);
    println!("{:?}", nodes);
}

fn parse_directions(line: &String) -> Vec<Direction> {
    line.chars().map(|c| Direction::parse(c)).collect()
}

fn parse_nodes(input: &Vec<String>) -> Vec<Node> {
    let mut nodes = Vec::<Node>::new();
    for line in &input[2..] {
        let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
        for (_, [n, l, r]) in re.captures_iter(line).map(|caps| caps.extract()) {
            nodes.push(Node {
                name: n.to_string(),
                left: l.to_string(),
                right: r.to_string(),
            });
        }
    }
    return nodes;
}
