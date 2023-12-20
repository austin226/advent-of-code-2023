use std::collections::HashMap;

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::common::get_input;

const BROADCASTER_NAME: &str = "broadcaster";

#[derive(Debug, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

trait ProcessPulse: std::fmt::Debug {
    /// Receive a pulse and maybe emit a pulse.
    fn process(&mut self, pulse: Pulse) -> Option<Pulse>;
}

#[derive(Debug, Default)]
struct FlipFlop {
    is_on: bool,
}

impl ProcessPulse for FlipFlop {
    fn process(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                if self.is_on {
                    self.is_on = false;
                    Some(Pulse::Low)
                } else {
                    self.is_on = true;
                    Some(Pulse::High)
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct Conjunction {
    ever_received_low: bool,
}

impl ProcessPulse for Conjunction {
    fn process(&mut self, pulse: Pulse) -> Option<Pulse> {
        if self.ever_received_low {
            Some(Pulse::High)
        } else {
            match pulse {
                Pulse::High => Some(Pulse::Low),
                Pulse::Low => {
                    self.ever_received_low = true;
                    Some(Pulse::High)
                }
            }
        }
    }
}

#[derive(Debug)]
struct Broadcaster;

impl ProcessPulse for Broadcaster {
    fn process(&mut self, pulse: Pulse) -> Option<Pulse> {
        Some(pulse)
    }
}

#[derive(Debug)]
struct Module {
    pulse_processor: Box<dyn ProcessPulse>,
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
            let caps = MODULE_REGEX.captures(line)?;
            let name = caps.get(2)?.as_str();
            let module_name = name.to_string();
            let pulse_processor: Box<dyn ProcessPulse> = if name == BROADCASTER_NAME {
                Box::new(Broadcaster)
            } else {
                let prefix = caps.get(1)?.as_str();
                match prefix {
                    "%" => Box::<FlipFlop>::default(),
                    "&" => Box::<Conjunction>::default(),
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
                    pulse_processor,
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
