use crate::common::get_input;
use regex::Regex;

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

pub fn run() {
    let input = get_input("src/day8/input0.txt");

    let left_right_instructions = input[0].chars();

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

    println!("{:?}", nodes);
}
