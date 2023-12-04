use crate::common::get_input;

pub fn run() {
    let input = get_input("src/day4/input0.txt");

    for line in input {
        let (winning, mine) = parse_line_numbers(line);
        
    }
}

fn parse_line_numbers(line: String) -> (Vec<i32>, Vec<i32>) {
    let numbers: Vec<&str> = line.split(": ").collect();
    let numbers: Vec<&str> = numbers[1].split(" | ").collect();
    let (winning, mine) = (numbers[0], numbers[1]);
    let winning = winning
        .split_whitespace()
        .map(|n| n.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mine = mine
        .split_whitespace()
        .map(|n| n.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    return (winning, mine);
}
