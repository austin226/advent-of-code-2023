// WIP - unsolved
use indicatif::ProgressBar;
use std::{collections::HashMap, ops::Range};

use crate::common::get_input;

#[derive(Clone, Debug)]
struct MatMapType {
    src: String,
    dst: String,
    maps: Vec<MatMap>,
}

impl MatMapType {
    fn convert(&self, key: u64) -> u64 {
        for map in self.maps.iter() {
            if (map.src_range_start..(map.src_range_start + map.range_len)).contains(&key) {
                return key - map.src_range_start + map.dst_range_start;
            }
        }
        // Any unmapped src corresponds to same dst number
        key
    }
}

#[derive(Clone, Debug)]
struct MatMap {
    dst_range_start: u64,
    src_range_start: u64,
    range_len: u64,
}

impl MatMap {
    fn contains(&self, n: u64) -> bool {
        (self.src_range_start..(self.src_range_start + self.range_len)).contains(&n)
    }

    fn src_end(&self) -> u64 {
        self.src_range_start + self.range_len
    }
}

pub fn run() {
    let input = get_input("src/day5/input1.txt");

    let seed_strs: Vec<&str> = input[0].split(": ").collect();
    let seed_pairs: Vec<u64> = seed_strs[1]
        .split_ascii_whitespace()
        .map(|n| n.to_string().parse::<u64>().unwrap())
        .collect();
    let mut seed_ranges: Vec<Range<u64>> = seed_pairs
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect();
    seed_ranges.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
    let seed_ranges = seed_ranges;

    let mut all_types = HashMap::new();
    let mut curr_map_type: Option<MatMapType> = None;
    for line in input.iter() {
        if line.trim() == "" {
            if let Some(curr_map_t) = curr_map_type {
                // This ends a map type
                let src = curr_map_t.src.clone();
                all_types.insert(src, curr_map_t.clone());
            }
            curr_map_type = None;
        } else if line.ends_with("map:") {
            // This starts a map type
            let line_split: Vec<&str> = line.split_ascii_whitespace().collect();
            let map_type_str: Vec<&str> = line_split[0].split_ascii_whitespace().collect();
            let map_type_tokens: Vec<&str> = map_type_str[0].split("-").collect();
            let map_from = map_type_tokens[0];
            let map_to = map_type_tokens[2];
            curr_map_type = Some(MatMapType {
                src: map_from.to_string(),
                dst: map_to.to_string(),
                maps: Vec::new(),
            });
        } else if curr_map_type.is_some() {
            // This adds a map to the type
            let map_nums: Vec<u64> = line
                .split_ascii_whitespace()
                .map(|n| n.to_string().parse::<u64>().unwrap())
                .collect();
            let map_type = curr_map_type.as_mut().unwrap();
            let map = MatMap {
                dst_range_start: map_nums[0],
                src_range_start: map_nums[1],
                range_len: map_nums[2],
            };
            map_type.maps.push(map);
        }
    }
    if let Some(curr_map_t) = curr_map_type {
        // This ends a map type
        let src = curr_map_t.src.clone();
        all_types.insert(src, curr_map_t.clone());
    }

    // Find the lowest "location" number that coresponds to any of the initial "seed"s
    let mut min_loc_num: Option<u64> = None;
    println!("{:?}", seed_ranges);
    let total_seeds = seed_ranges.iter().fold(0, |acc, r| acc + r.end - r.start);
    let bar = ProgressBar::new(total_seeds);

    for seed_range in seed_ranges {
        // TODO this approach is too slow!
        for seed in seed_range {
            bar.inc(1);
            let mut mat = seed;
            let mut map_type_name = "seed";
            while map_type_name != "location" {
                let map_type = all_types.get(map_type_name).unwrap();
                mat = map_type.convert(mat);
                map_type_name = map_type.dst.as_str();
            }

            let location_num = mat;
            if min_loc_num.is_none() {
                min_loc_num = Some(location_num);
            } else {
                min_loc_num = Some(std::cmp::min(min_loc_num.unwrap(), location_num));
            }
        }
    }
    bar.finish();

    println!("min={:?}", min_loc_num.unwrap());
}
