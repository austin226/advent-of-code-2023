use itertools::Itertools;

use crate::common::get_input;

#[derive(Debug)]
struct Graph {
    size: usize,
    matrix: Vec<Vec<u8>>,
}

impl Graph {
    fn new(input: &Vec<String>) -> Self
    {
        let size = input.len();
        let matrix: Vec<Vec<u8>> = input.iter().map(|row| {
            assert_eq!(size, row.len(), "Matrix must be square");
            row.chars().map(|c| {
                c.to_digit(10).expect("Parsing a digit") as u8
            }).collect_vec()
        }).collect_vec();
        Self { size, matrix }
    }
}

pub fn run() {
    let input = get_input("src/day17/input0.txt");
    let graph = Graph::new(&input);
    println!("{:?}", graph);
}
