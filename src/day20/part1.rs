use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::common::get_input;

const BROADCASTER_NAME: &str = "broadcaster";
const BUTTON_NAME: &str = "button";

#[derive(Debug, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

trait ProcessPulse: std::fmt::Debug {
    /// Receive a pulse and maybe emit a pulse.
    fn process(&mut self, pulse: Pulse, origin: String) -> Option<Pulse>;

    fn inputs(&mut self) -> &mut HashMap<String, Pulse>;

    fn connect(&mut self, input_name: String) {
        self.inputs().insert(input_name, Pulse::Low);
    }
}

#[derive(Debug, Default)]
struct FlipFlop {
    inputs: HashMap<String, Pulse>,
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

    fn inputs(&mut self) -> &mut HashMap<String, Pulse> {
        &mut self.inputs
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

    fn inputs(&mut self) -> &mut HashMap<String, Pulse> {
        &mut self.inputs
    }
}

#[derive(Debug, Default)]
struct Broadcaster {
    inputs: HashMap<String, Pulse>,
}

impl ProcessPulse for Broadcaster {
    fn process(&mut self, pulse: Pulse, _origin: String) -> Option<Pulse> {
        Some(pulse)
    }

    fn inputs(&mut self) -> &mut HashMap<String, Pulse> {
        &mut self.inputs
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
                Box::<Broadcaster>::default()
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

            let module_name = name.to_string();
            modules.insert(
                module_name,
                Module {
                    pulse_processor,
                    destinations,
                },
            );
        }

        // Connect inputs to modules
        let mut connections = Vec::new();
        for (module_name, module) in modules.iter() {
            for destination_name in module.destinations.iter() {
                connections.push((module_name.clone(), destination_name.clone()));
            }
        }
        for (origin_name, dest_name) in connections {
            if let Some(dest_module) = modules.get_mut(&dest_name) {
                let dest_processor = &mut dest_module.pulse_processor;
                dest_processor.connect(origin_name.clone());
            }
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

        // pulse, origin, destination
        let mut q = VecDeque::<(Pulse, String, String)>::new();
        q.push_back((
            Pulse::Low,
            BUTTON_NAME.to_string(),
            BROADCASTER_NAME.to_string(),
        ));

        while !q.is_empty() {
            let (pulse, origin_name, dest_name) = q.pop_front().expect("queue should not be empty");
            match pulse {
                Pulse::Low => {
                    self.low_pulses += 1;
                }
                Pulse::High => {
                    self.high_pulses += 1;
                }
            }

            // Log this pulse
            let pulse_name = match pulse {
                Pulse::Low => "low",
                Pulse::High => "high",
            };
            println!("{} -{pulse_name}-> {}", origin_name, dest_name);

            if let Some(dest) = self.modules.get_mut(&dest_name) {
                // Queue next pulses
                if let Some(out_pulse) = dest.pulse_processor.process(pulse, origin_name.clone()) {
                    // Apply out_pulse to all destinations
                    for next_dest_name in dest.destinations.iter() {
                        q.push_back((out_pulse, dest_name.clone(), next_dest_name.clone()));
                    }
                }
            }
        }
    }
}

pub fn run() {
    let input = get_input("src/day20/input1.txt");
    let mut system = System::parse(&input).expect("Failed to parse");

    let mut high_pulses = 0;
    let mut low_pulses = 0;
    for i in 1..=1000 {
        println!("Button press {i}");
        system.press_button();
        high_pulses += system.high_pulses;
        low_pulses += system.low_pulses;
        println!("{} high, {} low", system.high_pulses, system.low_pulses);
        println!("---");
    }
    println!("Total: {} high, {} low", high_pulses, low_pulses);
    println!("Answer: {}", high_pulses * low_pulses);
}
