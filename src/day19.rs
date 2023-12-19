use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
enum Rule {
    Cmp(Selector, Cmp, Target),
    Target(Target),
}

#[derive(Debug, Clone, Copy)]
enum Selector {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
enum Cmp {
    LessThan(usize),
    GreaterThan(usize),
}

#[derive(Debug, Clone)]
enum Target {
    SwitchWorkflow(String),
    Accept,
    Reject,
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Cmp {
    fn is_ok(&self, value: usize) -> bool {
        match self {
            Self::LessThan(v) => value < *v,
            Self::GreaterThan(v) => value > *v,
        }
    }

    fn invert(&self) -> Self {
        match self {
            Self::LessThan(v) => Self::GreaterThan(v - 1),
            Self::GreaterThan(v) => Self::LessThan(v + 1),
        }
    }
}

impl FromStr for Workflow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((name, rest)) = s.split_once('{') else {
            return Err(anyhow!("No '{{' found in Rule"));
        };
        let Some(rules_str) = rest.strip_suffix('}') else {
            return Err(anyhow!("Workflow doesn't end with '}}' ({:?})", s));
        };

        let mut rules = Vec::new();
        for rule_str in rules_str.split(',') {
            rules.push(rule_str.parse()?);
        }

        Ok(Self {
            name: name.to_string(),
            rules,
        })
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((condition, target)) = s.split_once(':') else {
            return Ok(Self::Target(s.parse()?));
        };

        let selector = match condition.chars().next() {
            Some('x') => Selector::X,
            Some('m') => Selector::M,
            Some('a') => Selector::A,
            Some('s') => Selector::S,
            s => return Err(anyhow!("Invalid selector {:?}", s)),
        };

        let cmp_value: usize = condition[2..].parse()?;
        let cmp = match condition.chars().nth(1) {
            Some('<') => Cmp::LessThan(cmp_value),
            Some('>') => Cmp::GreaterThan(cmp_value),
            op => return Err(anyhow!("Invalid operator {:?}", op)),
        };

        Ok(Self::Cmp(selector, cmp, target.parse()?))
    }
}

impl FromStr for Target {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "A" => Target::Accept,
            "R" => Target::Reject,
            _ => Target::SwitchWorkflow(s.to_string()),
        })
    }
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let part_values = s
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .ok_or_else(|| anyhow!("Part not surrounded by '{{' and '}}'"))?;

        let mut x = None;
        let mut m = None;
        let mut a = None;
        let mut s = None;
        for v in part_values.split(',') {
            let (attr, value_str) = v
                .split_once('=')
                .ok_or_else(|| anyhow!("Invalid part value {:?}", v))?;
            match attr {
                "x" => x = Some(value_str.parse()?),
                "m" => m = Some(value_str.parse()?),
                "a" => a = Some(value_str.parse()?),
                "s" => s = Some(value_str.parse()?),
                _ => return Err(anyhow!("Invalid part attribute {:?}", attr)),
            }
        }

        Ok(Self {
            x: x.unwrap_or(0),
            m: m.unwrap_or(0),
            a: a.unwrap_or(0),
            s: s.unwrap_or(0),
        })
    }
}

fn parse_input(s: &str) -> Result<(HashMap<String, Workflow>, Vec<Part>)> {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    let Some((workflows_str, parts_str)) = s.split_once("\n\n") else {
        return Err(anyhow!("Unable to split into groups of rules or parts"));
    };

    for workflow_str in workflows_str.lines() {
        let workflow: Workflow = workflow_str.parse()?;
        workflows.insert(workflow.name.clone(), workflow);
    }

    for part_str in parts_str.lines() {
        parts.push(part_str.parse()?);
    }

    Ok((workflows, parts))
}

fn part_a(workflows: &HashMap<String, Workflow>, parts: &[Part]) -> Result<usize> {
    /// Recursively follow the branching rules and determine if the given part is accepted
    fn is_ok(
        workflows: &HashMap<String, Workflow>,
        workflow_name: &str,
        part: &Part,
    ) -> Result<bool> {
        let Some(workflow) = workflows.get(workflow_name) else {
            return Err(anyhow!("Missing workflow {:?}", workflow_name));
        };
        for rule in workflow.rules.iter() {
            let target = match rule {
                Rule::Cmp(selector, cmp, target) => {
                    let value = match selector {
                        Selector::X => part.x,
                        Selector::M => part.m,
                        Selector::A => part.a,
                        Selector::S => part.s,
                    };

                    cmp.is_ok(value).then_some(target)
                }
                Rule::Target(t) => Some(t),
            };

            match target {
                Some(Target::SwitchWorkflow(wn)) => return is_ok(workflows, wn, part),
                Some(Target::Accept) => return Ok(true),
                Some(Target::Reject) => return Ok(false),
                None => (),
            }
        }
        Err(anyhow!(
            "Exhausted all rules for workflow {:?}",
            workflow_name
        ))
    }

    let mut sum = 0;
    for part in parts {
        if is_ok(workflows, "in", part)? {
            sum += part.x + part.m + part.a + part.s;
        }
    }
    Ok(sum)
}

fn part_b(workflows: &HashMap<String, Workflow>) -> Result<usize> {
    // Track all selectors and comparisons that eventually accepts a part
    let mut accepted_cmp_sequences = Vec::new();

    // Visit all branches
    let mut to_visit = Vec::new();
    to_visit.push(("in".to_string(), 0, Vec::new()));
    while let Some((workflow_name, idx, mut cmps)) = to_visit.pop() {
        let Some(workflow) = workflows.get(&workflow_name) else {
            return Err(anyhow!("Missing workflow {:?}", workflow_name));
        };

        // Assume it's rejected if we exhaust all rules in a workflow
        let Some(rule) = workflow.rules.get(idx) else {
            continue;
        };

        match rule {
            Rule::Cmp(selector, cmp, target) => {
                // Add the rule rejected path. We invert the comparison and visit the next rule in
                // the workflow
                let mut cmps_with_inverse = cmps.clone();
                cmps_with_inverse.push((*selector, cmp.invert()));
                to_visit.push((workflow_name.clone(), idx + 1, cmps_with_inverse));

                // Add the rule accepted path
                cmps.push((*selector, *cmp));
                match target {
                    Target::SwitchWorkflow(wn) => to_visit.push((wn.to_string(), 0, cmps)),
                    Target::Accept => accepted_cmp_sequences.push(cmps),
                    Target::Reject => (),
                }
            }
            Rule::Target(t) => match t {
                Target::SwitchWorkflow(wn) => to_visit.push((wn.to_string(), 0, cmps)),
                Target::Accept => accepted_cmp_sequences.push(cmps.clone()),
                Target::Reject => (),
            },
        }
    }

    // Find the number of combinations for each accepted sequence and add them up
    let mut sum = 0;
    for cmps in accepted_cmp_sequences {
        let mut possible_x: HashSet<usize> = (1..=4000).collect();
        let mut possible_m: HashSet<usize> = (1..=4000).collect();
        let mut possible_a: HashSet<usize> = (1..=4000).collect();
        let mut possible_s: HashSet<usize> = (1..=4000).collect();
        for (selector, cmp) in cmps {
            let value_domain = match selector {
                Selector::X => &mut possible_x,
                Selector::M => &mut possible_m,
                Selector::A => &mut possible_a,
                Selector::S => &mut possible_s,
            };
            value_domain.retain(|&v| cmp.is_ok(v));
        }
        sum += possible_x.len() * possible_m.len() * possible_a.len() * possible_s.len();
    }
    Ok(sum)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let instructions_str = std::fs::read_to_string(path)?;
    let (workflows, parts) = parse_input(&instructions_str)?;
    Ok((part_a(&workflows, &parts)?, part_b(&workflows)?.into()))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(19, 397_643, 132_392_981_697_081);

    const EXAMPLE_INPUT: &'static str = concat!(
        "px{a<2006:qkq,m>2090:A,rfg}\n",
        "pv{a>1716:R,A}\n",
        "lnx{m>1548:A,A}\n",
        "rfg{s<537:gd,x>2440:R,A}\n",
        "qs{s>3448:A,lnx}\n",
        "qkq{x<1416:A,crn}\n",
        "crn{x>2662:A,R}\n",
        "in{s<1351:px,qqz}\n",
        "qqz{s>2770:qs,m<1801:hdj,R}\n",
        "gd{a>3333:R,R}\n",
        "hdj{m>838:A,pv}\n",
        "\n",
        "{x=787,m=2655,a=1222,s=2876}\n",
        "{x=1679,m=44,a=2067,s=496}\n",
        "{x=2036,m=264,a=79,s=2244}\n",
        "{x=2461,m=1339,a=466,s=291}\n",
        "{x=2127,m=1623,a=2188,s=1013}\n",
    );

    #[test]
    fn test_part_a() {
        let (workflows, parts) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_a(&workflows, &parts).unwrap(), 19_114);
    }

    #[test]
    fn test_part_b() {
        let (workflows, _) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_b(&workflows).unwrap(), 167_409_079_868_000);
    }
}
