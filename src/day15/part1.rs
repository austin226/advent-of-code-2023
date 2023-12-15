use crate::common::get_input;

fn hash(step_input: &str) -> u32 {
    step_input
        .bytes()
        .fold(0, |acc, x| ((((acc as u32) + (x as u32)) * 17) % 256))
}

pub fn run() {
    let input = get_input("src/day15/input1.txt");
    let input_steps = input[0].split(',');
    let ans = input_steps.fold(0u32, |acc, x| acc + hash(x));
    println!("{ans}");
}
