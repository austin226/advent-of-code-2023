use crate::common::get_input;

struct Node {
    name: String,
    left: String,
    right: String,
}

pub fn run() {
    let input = get_input("src/day8/input0.txt");

    let left_right_instructions = input[0].chars();

    for line in &input[2..] {
        
        println!("{:?}", line);
    }
}
