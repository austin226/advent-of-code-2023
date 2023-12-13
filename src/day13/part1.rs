use crate::common::get_input;

pub fn run() {
    let input = get_input("src/day13/input0.txt");

    let mut patterns = Vec::new();
    let mut n_spaces = 0;
    for line in input {
        if patterns.is_empty() || n_spaces > (patterns.len() - 1) {
            patterns.push(Vec::new());
        }
        if line == "" {
            n_spaces += 1;
        } else {
            patterns[n_spaces].push(line);
        }
    }

    println!("{:?}", patterns);
}
