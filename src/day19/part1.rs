use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::common::get_input;

static PART_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}").unwrap());
static WORKFLOW_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-z]+)\{([^}]*)}").unwrap());
static RULE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([xmas])([<>])(\d+):(?:([a-z]+)|([AR]))|([a-z]+)|([AR])").unwrap());

enum Attribute {
    X,
    M,
    A,
    S,
}

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn parse(line: &str) -> Option<Self> {
        todo!()
    }

    fn get_attribute(&self, attribute: &Attribute) -> i32 {
        match attribute {
            Attribute::X => self.x,
            Attribute::M => self.m,
            Attribute::A => self.a,
            Attribute::S => self.s,
        }
    }
}

enum Condition {
    /// Condition always passes
    Always,

    /// Condition passes if attribute > threshold
    AttrGreaterThan {
        attribute: Attribute,
        threshold: i32,
    },

    /// Condition passes if attribute < threshold
    AttrLessThan {
        attribute: Attribute,
        threshold: i32,
    },
}

/// Next workflow, or a terminal destination
enum Destination {
    /// Terminal - either accept or reject the part
    Terminal(bool),

    /// Go to another workflow
    Next(Box<Workflow>),
}

struct Rule {
    /// If condition passes, go to destination
    condition: Condition,

    destination: Destination,
}

struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(line: &str) -> Option<Self> {
        todo!()
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<Workflow>, Vec<Part>) {
    let mut workflows = Vec::new();
    let mut parts = Vec::new();
    for line in input {
        if let Some(workflow) = Workflow::parse(line.as_str()) {
            workflows.push(workflow);
        } else if let Some(part) = Part::parse(line.as_str()) {
            parts.push(part);
        }
    }

    (workflows, parts)
}

pub fn run() {
    let input = get_input("src/day19/input0.txt");
    let (workflows, parts) = parse_input(&input);
}
