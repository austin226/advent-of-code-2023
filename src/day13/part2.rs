use crate::common::get_input;

fn hamming_distance(s1: &String, s2: &String) -> usize {
    s1.chars()
        .into_iter()
        .zip(s2.chars().into_iter())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn reflect_up_rows(pattern: &Vec<String>) -> Option<u32> {
    // println!("Check pattern: {:?}", pattern);
    for r in 1..pattern.len() {
        // Check if r is a point of symmetry
        let mut rem_smudges = 1;
        let mut is_symmetrical = true;
        let mut down_r = r;
        let mut up_r = r - 1;
        while down_r < pattern.len() {
            let dist = hamming_distance(&pattern[up_r], &pattern[down_r]);
            if dist > rem_smudges {
                is_symmetrical = false;
                break;
            }
            rem_smudges -= dist;
            if up_r == 0 {
                break;
            }
            down_r += 1;
            up_r -= 1;
        }
        if is_symmetrical && rem_smudges == 0 {
            // println!("Symmetrical at {r}");
            return Some(r as u32);
        }
    }
    return None;
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
    // println!("Check vert:");
    return reflect_up_rows(&vert_slices);
}

pub fn run() {
    let input = get_input("src/day13/input1.txt");

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
        .map(|pattern| {
            if let Some(r) = reflect_up_rows(pattern) {
                100 * r
            } else if let Some(c) = reflect_left_cols(pattern) {
                c
            } else {
                panic!("No symmetry for pattern: {:?}", pattern)
            }
        })
        .sum::<u32>();

    println!("{:?}", ans);
}
