use crate::common::get_input;

fn reflect_left_cols(pattern: &Vec<String>) -> Option<u32> {
    // TODO
    None
}

fn reflect_up_rows(pattern: &Vec<String>) -> Option<u32> {
    // TODO
    None
}

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

    let ans = patterns
        .iter()
        .map(
            |pattern| match (reflect_left_cols(pattern), reflect_up_rows(pattern)) {
                (Some(c), None) => c,
                (None, Some(r)) => 100 * r,
                _ => panic!("No symmetry for pattern: {:?}", pattern),
            },
        )
        .sum::<u32>();

    println!("{:?}", ans);
}
