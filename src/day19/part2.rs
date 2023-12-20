use std::collections::HashMap;
use std::ops::Range;

use once_cell::sync::Lazy;
use range_collections::range_set::{RangeSet, RangeSetRange};
use range_collections::RangeSet2;
use regex::Regex;

use crate::common::get_input;

const PART_RANGE: Range<i32> = 1..4001;

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

#[derive(Clone)]
struct PartRange {
    x: RangeSet2<i32>,
    m: RangeSet2<i32>,
    a: RangeSet2<i32>,
    s: RangeSet2<i32>,
}

fn sum_range_set(range_set: &RangeSet2<i32>) -> i32 {
    range_set
        .iter()
        .map(|range| match range {
            RangeSetRange::Range(range) => range.end - range.start,
            _ => panic!("Bad range"),
        })
        .sum()
}

impl PartRange {
    fn new() -> Self {
        Self {
            x: RangeSet::from(PART_RANGE),
            m: RangeSet::from(PART_RANGE),
            a: RangeSet::from(PART_RANGE),
            s: RangeSet::from(PART_RANGE),
        }
    }

    fn reduce(&mut self, attribute: Attribute, range: Range<i32>) {
        let range_set: RangeSet2<i32> = RangeSet::from(range);
        match attribute {
            Attribute::X => self.x.intersection_with(&range_set),
            Attribute::M => self.m.intersection_with(&range_set),
            Attribute::A => self.a.intersection_with(&range_set),
            Attribute::S => self.s.intersection_with(&range_set),
        };
    }

    fn intersect(&self, attribute: Attribute, range: Range<i32>) -> PartRange {
        let mut new_part_range = self.clone();
        new_part_range.reduce(attribute, range);
        new_part_range
    }

    fn subtract(&mut self, attribute: Attribute, range: Range<i32>) {
        let range_set: RangeSet2<i32> = RangeSet::from(range);
        match attribute {
            Attribute::X => self.x.difference_with(&range_set),
            Attribute::M => self.m.difference_with(&range_set),
            Attribute::A => self.a.difference_with(&range_set),
            Attribute::S => self.s.difference_with(&range_set),
        };
    }

    fn difference(&self, attribute: Attribute, range: Range<i32>) -> PartRange {
        let mut new_part_range = self.clone();
        new_part_range.subtract(attribute, range);
        new_part_range
    }

    fn size(&self) -> i64 {
        [&self.x, &self.m, &self.a, &self.s]
            .iter()
            .map(|rs| sum_range_set(rs))
            .fold(1, |acc, x| acc * x as i64)
    }

    fn is_empty(&self) -> bool {
        [&self.x, &self.m, &self.a, &self.s]
            .iter()
            .all(|rs| rs.is_empty())
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

    fn get_workflow(&self, workflow_name: &String) -> &Workflow {
        self.workflows
            .get(workflow_name)
            .unwrap_or_else(|| panic!("workflow {workflow_name} not found"))
    }

    /// Count all the possible parts that will be accepted.
    fn process(&self) -> i64 {
        let mut part_range = PartRange::new();
        self.process_part_range(&mut part_range, &"in".to_string());
        part_range.size()
    }

    /// Return the subset of the part range that will be accepted, starting in the given workflow.
    fn process_part_range(&self, part_range: &mut PartRange, workflow_name: &String) -> PartRange {
        // TODO maybe return the new part range? idk
        // Eventually, we only need a part range of accepted parts.
        // So, we can just drop any ranges that are rejected.
        let workflow = self.get_workflow(workflow_name);
        for rule in workflow.rules.iter() {
            match rule.condition {
                Condition::AttrLessThan {
                    attribute,
                    threshold,
                } => {
                    let attr_range = PART_RANGE.start..threshold;
                    let mut new_range = part_range.intersect(attribute, attr_range.clone());
                    if !new_range.is_empty() {
                        match rule.destination.clone() {
                            Destination::Next { workflow_name } => {
                                // self.process_part_range(&mut new_range, &workflow_name)
                                // TODO reduce part_range to intersect with new_range
                            }
                            Destination::Terminal { accept } => {
                                if accept {
                                    *part_range = new_range;
                                } else {
                                    *part_range = part_range.difference(attribute, attr_range);
                                }
                            }
                        }
                    }
                }
                Condition::AttrGreaterThan {
                    attribute,
                    threshold,
                } => {
                    let attr_range = (threshold + 1)..PART_RANGE.end;
                    // TODO repeat logic from AttrLessThan
                    todo!()
                }
                Condition::Always => todo!(),
            }
            // TODO repeat for other rules, reducing their respective part ranges
        }
        // TODO return merged part range of accepted
        todo!()
    }
}

pub fn run() {
    let input = get_input("src/day19/input0.txt");
    let system = System::parse(&input);
    let ans = system.process();
    println!("{ans}");
}
