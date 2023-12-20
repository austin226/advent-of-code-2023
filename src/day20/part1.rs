use std::collections::HashMap;

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::common::get_input;

const BROADCASTER_NAME: &str = "broadcaster";

#[derive(Debug)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    destinations: Vec<String>,
}

#[derive(Debug)]
struct System {
    modules: HashMap<String, Module>,
}

impl System {
    fn parse(input: &Vec<String>) -> Option<Self> {
        static MODULE_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"([%&]?)(\w+) -> (.*)").unwrap());
        let mut modules = HashMap::new();
        for line in input {
            let module_type: ModuleType;

            let caps = MODULE_REGEX.captures(line)?;
            let name = caps.get(2)?.as_str();
            let module_name = name.to_string();
            let module_type = if name == BROADCASTER_NAME {
                ModuleType::Broadcaster
            } else {
                let prefix = caps.get(1)?.as_str();
                match prefix {
                    "%" => ModuleType::FlipFlop,
                    "&" => ModuleType::Conjunction,
                    _ => return None,
                }
            };

            let destinations = caps
                .get(3)?
                .as_str()
                .split(", ")
                .map(|s| s.to_string())
                .collect_vec();

            modules.insert(
                module_name,
                Module {
                    module_type,
                    destinations,
                },
            );
        }

        Some(Self { modules })
    }
}

pub fn run() {
    let input = get_input("src/day20/input0.txt");
    let system = System::parse(&input).expect("Failed to parse");
    println!("{:?}", system);
}
