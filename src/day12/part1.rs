use itertools::Itertools;

use crate::common::get_input;

fn process_template(template: &str, nums: &[usize], min_start: usize) -> u32 {
    if nums.len() == 0 {
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
        n_possibilities += process_template(template, nums_r, start + my_num + 1);
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
    match min_start {
        Some(min_start) => {
            return process_template(template, &nums[..], min_start);
        }
        None => {
            return 0;
        }
    }
}

pub fn run() {
    let input = get_input("src/day12/input1.txt");

    let mut sum = 0;
    for line in input.iter() {
        let tokens = line.split_ascii_whitespace().collect_vec();
        let template = tokens[0];
        let nums: Vec<usize> = tokens[1]
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect_vec();

        sum += solution(template, &nums);
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
            assert_eq!(
                expected,
                solution(template, &nums),
                "template '{}'",
                template
            );
        }
    }
}
