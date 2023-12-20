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
}

impl System {
    fn parse(input: &Vec<String>) -> Self {
        let mut workflows = HashMap::new();

        for line in input {
            if let Some(workflow) = Workflow::parse(line.as_str()) {
                workflows.insert(workflow.name.clone(), workflow);
            }
        }

        Self { workflows }
    }
}

pub fn run() {
    let input = get_input("src/day19/input0.txt");
    let system = System::parse(&input);
    // let ans = system.process_parts();
    // println!("{ans}");
}
