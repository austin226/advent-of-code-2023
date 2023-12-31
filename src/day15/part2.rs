use std::fmt;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::common::get_input;

static STEP_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-z]+)((-)|(=)(\d+))").unwrap());

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

impl fmt::Debug for Lens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_length)
    }
}

impl Lens {
    fn focusing_power(&self, box_index: u8, slot_index: usize) -> u64 {
        (1 + box_index as u64) * (1 + slot_index as u64) * (self.focal_length as u64)
    }
}

#[derive(Clone)]
struct LensBox {
    index: u8,
    lenses: Vec<Lens>,
}

impl LensBox {
    fn new(index: u8) -> Self {
        Self {
            index,
            lenses: Vec::new(),
        }
    }

    fn add_or_replace(&mut self, label: &String, focal_length: u8) {
        for lens in self.lenses.iter_mut() {
            if lens.label == *label {
                // Replace lens if found
                lens.focal_length = focal_length;
                return;
            }
        }

        // Add lens if not found
        self.lenses.push(Lens {
            label: label.clone(),
            focal_length,
        });
    }

    fn remove(&mut self, label: &String) {
        let index = self.lenses.iter().position(|l| l.label == *label);
        if let Some(index) = index {
            self.lenses.remove(index);
        }
    }

    fn is_empty(&self) -> bool {
        self.lenses.is_empty()
    }

    fn focusing_power(&self) -> u64 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, lens)| lens.focusing_power(self.index, i))
            .sum::<u64>()
    }
}

impl fmt::Debug for LensBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lenses = String::new();
        for lens in self.lenses.iter() {
            lenses = format!("{} {:?}", lenses, lens);
        }
        write!(f, "Box {}:{}", self.index, lenses)
    }
}

#[derive(Debug)]
enum StepType {
    Add(u8),
    Remove,
}

#[derive(Debug)]
struct Step {
    label: String,
    step_type: StepType,
}

impl Step {
    fn parse(step_str: &str) -> Self {
        let re = &STEP_REGEX;
        let caps = re.captures(step_str).unwrap();
        let label = caps.get(1).unwrap().as_str();
        let minus = caps.get(3).and_then(|f| Some(f.as_str()));
        let eq = caps.get(4).and_then(|f| Some(f.as_str()));
        let focal_length = caps
            .get(5)
            .and_then(|f| Some(f.as_str().parse::<u8>().unwrap()));

        if let Some(minus) = minus {
            return Self {
                label: label.to_string(),
                step_type: StepType::Remove,
            };
        } else if let (Some(eq), Some(focal_length)) = (eq, focal_length) {
            return Self {
                label: label.to_string(),
                step_type: StepType::Add(focal_length),
            };
        } else {
            panic!("Invalid step_str: {step_str}");
        }
    }
}

fn hash(step_input: &str) -> u8 {
    step_input
        .bytes()
        .fold(0, |acc, x| ((((acc as u32) + (x as u32)) * 17) % 256)) as u8
}

pub fn run() {
    let input = get_input("src/day15/input1.txt");
    let input_steps = input[0].split(',');

    let mut lens_boxes = Vec::new();
    for i in 0..=255 {
        lens_boxes.push(LensBox::new(i));
    }

    let steps = input_steps.map(|step_str| Step::parse(step_str));
    for step in steps {
        let box_key = hash(&step.label);
        let lens_box = &mut lens_boxes[box_key as usize];
        match step.step_type {
            StepType::Add(focal_length) => {
                lens_box.add_or_replace(&step.label, focal_length);
            }
            StepType::Remove => {
                lens_box.remove(&step.label);
            }
        }
    }

    let mut focusing_power = 0;
    for b in lens_boxes {
        if !b.is_empty() {
            focusing_power += b.focusing_power();
            println!("{:?}", b);
        }
    }

    println!("{}", focusing_power);
}
