use once_cell::sync::Lazy;
use regex::Regex;

use crate::common::get_input;

static STEP_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-z]+)((-)|(=)(\d+))").unwrap());

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Debug)]
struct LensBox {
    lenses: Vec<Lens>,
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

fn hash(step_input: &str) -> u32 {
    step_input
        .bytes()
        .fold(0, |acc, x| ((((acc as u32) + (x as u32)) * 17) % 256))
}

pub fn run() {
    let input = get_input("src/day15/input0.txt");
    let input_steps = input[0].split(',');

    let steps = input_steps.map(|step_str| Step::parse(step_str));
    for step in steps {
        //
    }
}
