use crate::common::get_input;

fn reflect_up_rows(pattern: &Vec<String>) -> Option<u32> {
    // TODO
    None
}

fn reflect_left_cols(pattern: &Vec<String>) -> Option<u32> {
    // Convert pattern into vertical slices
    let mut vert_slices = Vec::<String>::new();
    vert_slices.reserve(pattern[0].len());
    for c in 0..pattern[0].len() {
        let mut slice = Vec::new();
        slice.reserve(pattern.len());
        for r in 0..pattern.len() {
            let chr = pattern[r].chars().nth(c).unwrap();
            slice.push(chr);
        }
        vert_slices.push(slice.iter().collect());
    }

    // Find horizontal symmetry value of the rotated pattern
    return reflect_up_rows(&vert_slices);
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
