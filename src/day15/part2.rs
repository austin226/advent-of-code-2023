use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::common::get_input;

static STEP_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-z]+)((-)|(=)(\d+))").unwrap());

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Debug, Clone)]
struct LensBox {
    lenses: Vec<Lens>,
}

impl LensBox {
    fn new() -> Self {
        Self { lenses: Vec::new() }
    }

    fn remove(&mut self, label: &String) {
        let index = self.lenses.iter().position(|l| l.label == *label);
        if let Some(index) = index {
            self.lenses.remove(index);
        }
    }
}

#[derive(Debug)]
enum StepType {
    Add(i32),
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
        let focal = caps
            .get(5)
            .and_then(|f| Some(f.as_str().parse::<i32>().unwrap()));

        if let Some(minus) = minus {
            return Self {
                label: label.to_string(),
                step_type: StepType::Remove,
            };
        } else if let (Some(eq), Some(focal)) = (eq, focal) {
            return Self {
                label: label.to_string(),
                step_type: StepType::Add(focal),
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
    let input = get_input("src/day15/input0.txt");
    let input_steps = input[0].split(',');

    let mut lens_boxes = vec![LensBox::new(); 256];
    let steps = input_steps.map(|step_str| Step::parse(step_str));
    for step in steps {
        let box_key = hash(&step.label);
        let lens_box = &mut lens_boxes[box_key as usize];
        match step.step_type {
            StepType::Add(_) => {}
            StepType::Remove => {
                lens_box.remove(&step.label);
            }
        }
    }

    println!("{:?}", lens_boxes);
}
