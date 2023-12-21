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
    fn process(&mut self, pulse: Pulse, origin: String) -> Option<Pulse>;
}

#[derive(Debug, Default)]
struct FlipFlop {
    is_on: bool,
}

impl ProcessPulse for FlipFlop {
    fn process(&mut self, pulse: Pulse, _origin: String) -> Option<Pulse> {
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
    /// Map of each input to their last pulse.
    inputs: HashMap<String, Pulse>,
}

impl ProcessPulse for Conjunction {
    fn process(&mut self, pulse: Pulse, origin: String) -> Option<Pulse> {
        match pulse {
            Pulse::Low | Pulse::High => {
                self.inputs.insert(origin, pulse);
            }
        }
        if self.inputs.values().all(|p| match p {
            Pulse::Low => false,
            Pulse::High => true,
        }) {
            // All inputs are High
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }
}

#[derive(Debug)]
struct Broadcaster;

impl ProcessPulse for Broadcaster {
    fn process(&mut self, pulse: Pulse, _origin: String) -> Option<Pulse> {
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
                name.to_string(),
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

    fn press_button(&mut self) {
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
            if let Some(module) = self.modules.get_mut(&module_name) {
                if let Some(out_pulse) = module
                    .pulse_processor
                    .process(in_pulse, module_name.clone())
                {
                    // Apply out_pulse to all destinations
                    for dest_module_name in module.destinations.iter() {
                        let pulse_name = match out_pulse {
                            Pulse::Low => "low",
                            Pulse::High => "high",
                        };
                        println!(
                            "{} -{pulse_name}-> {}",
                            module_name.clone(),
                            dest_module_name.clone()
                        );
                        q.push_back((out_pulse, dest_module_name.clone()));
                    }
                }
            }
        }
    }
}

pub fn run() {
    let input = get_input("src/day20/input2.txt");
    let mut system = System::parse(&input).expect("Failed to parse");
    system.press_button();
    println!("{} high, {} low", system.high_pulses, system.low_pulses);
}
