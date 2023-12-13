use itertools::Itertools;

use crate::common::get_input;

// WIP - not solved yet

fn process_template(template: &str, nums: &[usize], min_start: usize, result_str: String) -> u32 {
    if nums.len() == 0 {
        let mut str_builder = result_str.clone();
        for i in str_builder.len()..=template.len() {
            str_builder += ".";
        }
        // Check the template against str_builder
        for i in 0..template.len() {
            let t = template.chars().nth(i).unwrap();
            let my = str_builder.chars().nth(i).unwrap();
            if t == '.' && my == '#' || t == '#' && my != '#' {
                // println!("Mismatch - template={template}, str = {str_builder}");
                return 0;
            }
        }

        // println!("{}", str_builder);
        return 1;
    }
    let my_num = nums[0];
    if min_start + my_num - 1 >= template.len() {
        return 0;
    }

    let nums_r = &nums[1..];
    let space_r = nums_r.iter().fold(0, |s: usize, n| s + n + 1);

    let max_start = template.len() - space_r - my_num;
    let mut n_possibilities = 0;
    for start in min_start..=max_start {
        // Test a position for this number
        if template[start..(start + my_num)].contains(".") {
            // This is an invalid position
            continue;
        }

        if start + my_num < template.len() && template.chars().nth(start + my_num) == Some('#') {
            // This is an invalid position
            continue;
        }

        if start > 0 && template.chars().nth(start - 1) == Some('#') {
            // This is an invalid position
            continue;
        }

        // println!(
        //     "{:?} can go at {:?}..={:?} in {:?}, followed by {:?}",
        //     my_num,
        //     start,
        //     start + my_num - 1,
        //     template,
        //     nums_r
        // );

        // Process the rest of the numbers after locking in this position
        let mut str_builder = result_str.clone();
        for i in min_start..start {
            str_builder += ".";
        }
        for i in start..(start + my_num) {
            str_builder += "#";
        }
        str_builder += ".";
        n_possibilities += process_template(template, nums_r, start + my_num + 1, str_builder);
    }
    return n_possibilities;
}

fn solution(template: &str, nums: &[usize]) -> u32 {
    // Find leftmost point where we can start
    let min_start = template
        .char_indices()
        .into_iter()
        .find(|(_, c)| *c != '.')
        .map(|(i, _)| i);
    let mut str_builder = "".to_owned();
    match min_start {
        Some(min_start) => {
            for i in 0..min_start {
                str_builder += ".";
            }
            return process_template(template, &nums[..], min_start, str_builder);
        }
        None => {
            return 0;
        }
    }
}

fn unfold_template(template: String) -> String {
    const N_COPIES: usize = 5;
    let mut unfolded = template.clone();
    for i in 1..N_COPIES {
        unfolded.push_str("?");
        unfolded.push_str(&template);
    }
    return unfolded;
}

fn unfold_nums(nums: Vec<usize>) -> Vec<usize> {
    const N_COPIES: usize = 5;
    let mut unfolded = Vec::new();
    unfolded.reserve(nums.len() * N_COPIES);
    for _ in 0..N_COPIES {
        for n in nums.iter() {
            unfolded.push(*n);
        }
    }
    return unfolded;
}

pub fn run() {
    let input = get_input("src/day12/input0.txt");

    let mut sum = 0;
    for line in input.iter() {
        let tokens = line.split_ascii_whitespace().collect_vec();
        let template = unfold_template(tokens[0].to_string());
        let nums: Vec<usize> = tokens[1]
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect_vec();
        let nums = unfold_nums(nums);

        sum += solution(&template, &nums);
    }

    println!("{sum}");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_solution() {
        for (template, nums, expected) in [
            ("???.###", vec![1, 1, 3], 1),
            (".??..??...?##.", vec![1, 1, 3], 4),
            ("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6], 1),
            ("????.#...#...", vec![4, 1, 1], 1),
            ("????.######..#####.", vec![1, 6, 5], 4),
            ("?###????????", vec![3, 2, 1], 10),
            ("....", vec![1, 1, 1], 0),
            ("#", vec![2], 0),
            ("?####????###?###???.", vec![4, 9], 3),
        ] {
            println!("Template: {template}");
            let sol = solution(template, &nums);
            assert_eq!(expected, sol, "template '{}'", template);
            println!("{sol}\n");
        }
    }
}
