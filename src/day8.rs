use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::path::Path;

enum LR {
    Left,
    Right,
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

#[allow(clippy::type_complexity)]
fn parse_input(s: &str) -> Result<(Vec<LR>, HashMap<String, (String, String)>)> {
    let Some((lr_str, map_str)) = s.split_once("\n\n") else {
        return Err(anyhow!("Can't find two separate blocks in map"));
    };

    let steps = lr_str
        .chars()
        .map(|c| match c {
            'L' => Ok(LR::Left),
            'R' => Ok(LR::Right),
            _ => Err(anyhow!("Unknown step type {:?}", c)),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let map = map_str
        .lines()
        .map(|l| {
            let Some((src_str, dst_str)) = l.split_once(" = (") else {
                return Err(anyhow!("Invalid map line {:?}", l));
            };
            let Some((left_dst_str, right_dst_str)) = dst_str[..dst_str.len() - 1].split_once(", ")
            else {
                return Err(anyhow!("Invalid destination {:?}", dst_str));
            };
            Ok((
                src_str.to_string(),
                (left_dst_str.to_string(), right_dst_str.to_string()),
            ))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;
    Ok((steps, map))
}

fn follow_steps(
    steps: &[LR],
    map: &HashMap<String, (String, String)>,
    src: &str,
    dst: &str,
) -> Result<Option<usize>> {
    let mut curr = src;
    let mut seen_nodes = HashSet::new();
    for (num_steps, step) in steps.iter().cycle().enumerate() {
        if num_steps > 0 && curr == dst {
            return Ok(Some(num_steps));
        }

        // Detect cycles
        if !seen_nodes.insert((num_steps % steps.len(), curr)) {
            return Ok(None);
        }

        let Some((l, r)) = map.get(curr) else {
            return Err(anyhow!("Can't find {:?} in map", curr));
        };
        match step {
            LR::Left => curr = l,
            LR::Right => curr = r,
        }
    }
    unreachable!();
}

fn follow_ghost_steps(steps: &[LR], map: &HashMap<String, (String, String)>) -> Result<usize> {
    let mut a_to_z_steps: HashMap<(&str, &str), usize> = HashMap::new();
    for src in map.keys() {
        if !src.ends_with('A') {
            continue;
        }
        for dst in map.keys() {
            if !dst.ends_with('Z') {
                continue;
            }
            let Some(num_steps) = follow_steps(steps, map, src, dst)? else {
                continue;
            };
            a_to_z_steps.insert((src, dst), num_steps);
        }
    }

    let mut running_lcm = 1;
    let mut steps_until_aligned = 1;
    for ((_, from_to), offset) in a_to_z_steps.iter() {
        let Some(steps_in_cycle) = follow_steps(steps, map, from_to, from_to)? else {
            return Err(anyhow!("We expect ??Z to ??Z always have a valid path"));
        };
        while (steps_until_aligned + offset) % steps_in_cycle != 0 {
            steps_until_aligned += running_lcm;
        }
        running_lcm = lcm(running_lcm, steps_in_cycle);
    }
    Ok(steps_until_aligned)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = std::fs::read_to_string(path)?;
    let (steps, map) = parse_input(&file)?;
    Ok((
        follow_steps(&steps, &map, "AAA", "ZZZ")?.unwrap(),
        follow_ghost_steps(&steps, &map)?.into(),
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = concat!(
            "RL\n",
            "\n",
            "AAA = (BBB, CCC)\n",
            "BBB = (DDD, EEE)\n",
            "CCC = (ZZZ, GGG)\n",
            "DDD = (DDD, DDD)\n",
            "EEE = (EEE, EEE)\n",
            "GGG = (GGG, GGG)\n",
            "ZZZ = (ZZZ, ZZZ)\n",
        );
        let (steps, map) = parse_input(input).unwrap();
        assert_eq!(
            follow_steps(&steps, &map, "AAA", "ZZZ").unwrap().unwrap(),
            2
        );
    }

    #[test]
    fn test_part_b() {
        let input = concat!(
            "LR\n",
            "\n",
            "11A = (11B, XXX)\n",
            "11B = (XXX, 11Z)\n",
            "11Z = (11B, XXX)\n",
            "22A = (22B, XXX)\n",
            "22B = (22C, 22C)\n",
            "22C = (22Z, 22Z)\n",
            "22Z = (22B, 22B)\n",
            "XXX = (XXX, XXX)\n",
        );
        let (steps, map) = parse_input(input).unwrap();
        assert_eq!(follow_ghost_steps(&steps, &map).unwrap(), 6);
    }
}
