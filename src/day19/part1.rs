use rayon::prelude::*;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::common::get_input;

#[derive(Debug, Copy, Clone)]
enum Attribute {
    X,
    M,
    A,
    S,
}

impl Attribute {
    fn parse(attr_str: &str) -> Option<Attribute> {
        match attr_str {
            "x" => Some(Attribute::X),
            "m" => Some(Attribute::M),
            "a" => Some(Attribute::A),
            "s" => Some(Attribute::S),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn parse(line: &str) -> Option<Self> {
        static PART_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}").unwrap());
        let re = &PART_REGEX;
        let caps = re.captures(line)?;
        let x = caps.get(1)?.as_str().parse::<i32>().ok()?;
        let m = caps.get(2)?.as_str().parse::<i32>().ok()?;
        let a = caps.get(3)?.as_str().parse::<i32>().ok()?;
        let s = caps.get(4)?.as_str().parse::<i32>().ok()?;

        Some(Self { x, m, a, s })
    }

    fn get_attribute(&self, attribute: &Attribute) -> i32 {
        match attribute {
            Attribute::X => self.x,
            Attribute::M => self.m,
            Attribute::A => self.a,
            Attribute::S => self.s,
        }
    }

    fn rating(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }

    fn meets_condition(&self, condition: &Condition) -> bool {
        match condition {
            Condition::Always => true,
            Condition::AttrGreaterThan {
                attribute,
                threshold,
            } => self.get_attribute(attribute) > *threshold,
            Condition::AttrLessThan {
                attribute,
                threshold,
            } => self.get_attribute(attribute) < *threshold,
        }
    }
}

#[derive(Debug)]
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
#[derive(Debug, Clone)]
enum Destination {
    /// Terminal - either accept or reject the part
    Terminal { accept: bool },

    /// Go to another workflow by name
    Next { workflow_name: String },
}

#[derive(Debug)]
struct Rule {
    /// If condition passes, go to destination
    condition: Condition,

    destination: Destination,
}

impl Rule {
    fn parse(rule_str: &str) -> Option<Self> {
        static RULE_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"([xmas])([<>])(\d+):(?:([a-z]+)|([AR]))|([a-z]+)|([AR])").unwrap()
        });
        let re = &RULE_REGEX;
        let caps = re.captures(rule_str)?;

        let m_attr_name = caps.get(1);
        let m_comparator = caps.get(2);
        let m_threshold = caps.get(3);
        let m_dest_workflow = caps.get(4);
        let m_dest_terminal = caps.get(5);
        let m_uncond_workflow = caps.get(6);
        let m_uncond_terminal = caps.get(7);

        match (
            m_attr_name,
            m_comparator,
            m_threshold,
            m_dest_workflow,
            m_dest_terminal,
            m_uncond_workflow,
            m_uncond_terminal,
        ) {
            (Some(m_attr_name), Some(m_comparator), Some(m_threshold), _, _, _, _) => {
                let attribute = Attribute::parse(m_attr_name.as_str())?;
                let is_gt = match m_comparator.as_str() {
                    ">" => true,
                    "<" => false,
                    _ => {
                        return None;
                    }
                };
                let threshold = m_threshold.as_str().parse::<i32>().ok()?;
                let condition = if is_gt {
                    Condition::AttrGreaterThan {
                        attribute,
                        threshold,
                    }
                } else {
                    Condition::AttrLessThan {
                        attribute,
                        threshold,
                    }
                };

                let destination = match (m_dest_workflow, m_dest_terminal) {
                    (Some(m_dest_workflow), _) => {
                        // Conditional rule that leads to a workflow
                        let workflow_name = m_dest_workflow.as_str().to_string();
                        Some(Destination::Next { workflow_name })
                    }
                    (_, Some(m_dest_terminal)) => {
                        // Conditional rule that leads to a terminal
                        let accept = match m_dest_terminal.as_str() {
                            "A" => true,
                            "R" => false,
                            _ => {
                                return None;
                            }
                        };
                        Some(Destination::Terminal { accept })
                    }
                    _ => None,
                }?;

                Some(Rule {
                    condition,
                    destination,
                })
            }
            (_, _, _, _, _, Some(m_uncond_workflow), _) => {
                // Unconditional rule that leads to a workflow
                let workflow_name = m_uncond_workflow.as_str().to_string();
                Some(Rule {
                    condition: Condition::Always,
                    destination: Destination::Next { workflow_name },
                })
            }
            (_, _, _, _, _, _, Some(m_uncond_terminal)) => {
                // Unconditional rule that leads to a terminal
                // TODO move into parser for Destination
                let accept = match m_uncond_terminal.as_str() {
                    "A" => true,
                    "R" => false,
                    _ => {
                        return None;
                    }
                };
                Some(Rule {
                    condition: Condition::Always,
                    destination: Destination::Terminal { accept },
                })
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(line: &str) -> Option<Self> {
        static WORKFLOW_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"([a-z]+)\{([^}]*)}").unwrap());

        let re = &WORKFLOW_REGEX;
        let caps = re.captures(line)?;
        let name = caps.get(1)?.as_str();
        let rule_strs = caps.get(2)?.as_str();

        let mut rules = Vec::new();
        for rule_str in rule_strs.split(',') {
            rules.push(Rule::parse(rule_str)?);
        }

        Some(Self {
            name: name.to_string(),
            rules,
        })
    }
}

struct System {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl System {
    fn parse(input: &Vec<String>) -> Self {
        let mut workflows = HashMap::new();
        let mut parts = Vec::new();

        for line in input {
            if let Some(workflow) = Workflow::parse(line.as_str()) {
                workflows.insert(workflow.name.clone(), workflow);
            } else if let Some(part) = Part::parse(line.as_str()) {
                parts.push(part);
            }
        }

        Self { workflows, parts }
    }

    /// Return the sum of all accepted parts.
    fn process_parts(&self) -> i32 {
        self.parts
            .par_iter()
            // .iter()
            .filter(|part| self.process_part(part))
            .map(|part| part.rating())
            .sum()
    }

    /// Return whether the part is accepted.
    fn process_part(&self, part: &Part) -> bool {
        // println!("Process part {:?}", part);
        let mut current_workflow_name = "in".to_string();

        loop {
            // println!("Workflow {current_workflow_name}");
            let workflow = self
                .workflows
                .get(&current_workflow_name)
                .unwrap_or_else(|| panic!("workflow {current_workflow_name} not found"));
            for rule in workflow.rules.iter() {
                // println!("Checking rule {:?}", rule);
                if part.meets_condition(&rule.condition) {
                    match rule.destination.clone() {
                        Destination::Terminal { accept: true } => return true,
                        Destination::Terminal { accept: false } => return false,
                        Destination::Next { workflow_name } => {
                            current_workflow_name = workflow_name;
                            break;
                        }
                    }
                }
            }
        }
    }
}

pub fn run() {
    let input = get_input("src/day19/input0.txt");
    let system = System::parse(&input);
    let ans = system.process_parts();
    println!("{ans}");
}
