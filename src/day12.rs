use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Condition {
    Unknown,
    Operational,
    Damaged,
}

type Record = (Vec<Condition>, Vec<usize>);

fn parse_record(s: &str) -> Result<Record> {
    let Some((springs_str, groups_str)) = s.split_once(' ') else {
        return Err(anyhow!(
            "Unable to separate springs and group specification"
        ));
    };

    let mut springs = Vec::new();
    for c in springs_str.chars() {
        let condition = match c {
            '?' => Ok(Condition::Unknown),
            '.' => Ok(Condition::Operational),
            '#' => Ok(Condition::Damaged),
            _ => Err(anyhow!("Unknown spring condition {:?}", c)),
        };
        springs.push(condition?);
    }
    let damaged_groups = groups_str
        .split(',')
        .map(|n| n.parse())
        .collect::<Result<Vec<_>, _>>()?;
    Ok((springs, damaged_groups))
}

// This has a horrible signature because Rust doesn't really have anything like Python's LRU cache
// :(
fn memoized_num_cfgs(
    memo: &mut HashMap<(Vec<Condition>, Vec<usize>, usize), usize>,
    springs: &[Condition],
    cfg: &[usize],
    damaged_streak: usize,
) -> usize {
    // Try to retrieve the output from the cache
    let cache_key = (springs.to_vec(), cfg.to_vec(), damaged_streak);
    if let Some(&n) = memo.get(&cache_key) {
        return n;
    }

    // We had a cache miss and need to compute the number of configs
    let num_cfgs = {
        let max_damaged_streak = cfg.first().copied().unwrap_or(0);
        match springs.get(0).copied() {
            Some(Condition::Unknown) => {
                // Branch out to try all options
                let mut when_operational = springs.to_vec();
                when_operational[0] = Condition::Operational;

                let mut when_damaged = springs.to_vec();
                when_damaged[0] = Condition::Damaged;

                memoized_num_cfgs(memo, &when_operational, cfg, damaged_streak)
                    + memoized_num_cfgs(memo, &when_damaged, cfg, damaged_streak)
            }
            Some(Condition::Operational) => {
                if damaged_streak == 0 {
                    memoized_num_cfgs(memo, &springs[1..], cfg, 0)
                } else if damaged_streak != max_damaged_streak {
                    0
                } else {
                    memoized_num_cfgs(memo, &springs[1..], &cfg[1..], 0)
                }
            }
            Some(Condition::Damaged) => {
                if damaged_streak >= max_damaged_streak {
                    0
                } else {
                    memoized_num_cfgs(memo, &springs[1..], cfg, damaged_streak + 1)
                }
            }
            None => {
                // We have reached the end of the list of springs, so we must either have no damage
                // streak or exactly fulfill the current damage_streak
                usize::from(cfg.len() <= 1 && max_damaged_streak == damaged_streak)
            }
        }
    };

    // Cache the result and return it
    memo.insert(cache_key, num_cfgs);
    num_cfgs
}

fn part_a(records: &[Record]) -> usize {
    let mut memo = HashMap::new();
    let mut num_cfgs = 0;
    for (conditions, cfg) in records.iter() {
        num_cfgs += memoized_num_cfgs(&mut memo, conditions, cfg, 0);
    }
    num_cfgs
}

fn part_b(records: &[Record]) -> usize {
    let mut memo = HashMap::new();
    let mut num_cfgs = 0;
    for (conditions, cfg) in records.iter() {
        let mut extended_conditions = conditions.clone();
        let mut extended_cfg = cfg.clone();
        for _ in 0..4 {
            extended_conditions.push(Condition::Unknown);
            extended_conditions.extend(conditions.iter().copied());
            extended_cfg.extend(cfg.iter().copied());
        }
        num_cfgs += memoized_num_cfgs(&mut memo, &extended_conditions, &extended_cfg, 0);
    }
    num_cfgs
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let records = BufReader::new(file)
        .lines()
        .map(|lr| parse_record(&lr?))
        .collect::<Result<Vec<_>, _>>()?;
    Ok((part_a(&records), part_b(&records).into()))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(12, 7670, 157_383_940_585_037);

    fn example_input() -> Vec<Record> {
        [
            parse_record("???.### 1,1,3").unwrap(),
            parse_record(".??..??...?##. 1,1,3").unwrap(),
            parse_record("?#?#?#?#?#?#?#? 1,3,1,6").unwrap(),
            parse_record("????.#...#... 4,1,1").unwrap(),
            parse_record("????.######..#####. 1,6,5").unwrap(),
            parse_record("?###???????? 3,2,1").unwrap(),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&example_input()), 21);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&example_input()), 525152);
    }
}
