use std::collections::{HashMap, VecDeque};

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
    low_pulses: i32,
    high_pulses: i32,
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

        Some(Self {
            modules,
            low_pulses: 0,
            high_pulses: 0,
        })
    }

    fn simulate(&mut self) {
        self.low_pulses = 0;
        self.high_pulses = 0;

        // Holds pulses and the modules to which they will be applied.
        let mut q = VecDeque::<(Pulse, String)>::new();
        q.push_back((Pulse::Low, BROADCASTER_NAME.to_string()));

        while !q.is_empty() {
            let (in_pulse, module_name) = q.pop_front().expect("queue should not be empty");
            match in_pulse {
                Pulse::Low => {
                    self.low_pulses += 1;
                }
                Pulse::High => {
                    self.high_pulses += 1;
                }
            }
            let module = self
                .modules
                .get_mut(&module_name)
                .unwrap_or_else(|| panic!("Module {module_name} not found"));
            if let Some(out_pulse) = module.pulse_processor.process(in_pulse) {
                // Apply out_pulse to all destinations
                for dest_module_name in module.destinations.iter() {
                    q.push_back((out_pulse, dest_module_name.clone()));
                }
            }
        }
    }
}

pub fn run() {
    let input = get_input("src/day20/input0.txt");
    let system = System::parse(&input).expect("Failed to parse");
    println!("{} high, {} low", system.high_pulses, system.low_pulses);
}
