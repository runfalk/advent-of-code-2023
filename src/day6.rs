use anyhow::{anyhow, Result};
use std::path::Path;

fn parse_races(s: &str) -> Result<Vec<(usize, usize)>> {
    let mut lines = s.lines();
    let Some(time_str) = lines.next().filter(|l| l.starts_with("Time: ")) else {
        return Err(anyhow!("Unable to find times"));
    };
    let Some(distance_str) = lines.next().filter(|l| l.starts_with("Distance: ")) else {
        return Err(anyhow!("Unable to find distances"));
    };

    let times = time_str
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse())
        .collect::<Result<Vec<usize>, _>>()?;
    let distances = distance_str
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse())
        .collect::<Result<Vec<usize>, _>>()?;
    Ok(times.into_iter().zip(distances).collect())
}

fn num_winning_ways(time: usize, distance_to_beat: usize) -> usize {
    let mut num_ways = 0;
    for hold_time in 1..time {
        if hold_time * (time - hold_time) > distance_to_beat {
            num_ways += 1;
        }
    }
    num_ways
}

fn concat_usize(nums: impl Iterator<Item = usize>) -> usize {
    nums.fold(0, |acc, n| {
        acc * 10usize.pow(n.checked_ilog10().unwrap_or(0) + 1) + n
    })
}

fn part_a(races: &[(usize, usize)]) -> usize {
    races
        .iter()
        .copied()
        .map(|(t, d)| num_winning_ways(t, d))
        .product()
}

fn part_b(races: &[(usize, usize)]) -> usize {
    let time = concat_usize(races.iter().map(|(t, _)| *t));
    let distance = concat_usize(races.iter().map(|(_, d)| *d));
    num_winning_ways(time, distance)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = std::fs::read_to_string(path)?;
    let races = parse_races(&file)?;

    Ok((part_a(&races), part_b(&races).into()))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(6, 1_710_720, 35_349_468);

    const EXAMPLE_INPUT: &'static str =
        concat!("Time:      7  15   30\n", "Distance:  9  40  200\n",);

    fn example_input() -> Vec<(usize, usize)> {
        parse_races(EXAMPLE_INPUT).unwrap()
    }

    #[test]
    fn test_num_winning_ways() {
        assert_eq!(num_winning_ways(7, 9), 4);
        assert_eq!(num_winning_ways(15, 40), 8);
        assert_eq!(num_winning_ways(30, 200), 9);
    }

    #[test]
    fn test_concat_usize() {
        assert_eq!(concat_usize([1, 10, 100].into_iter()), 110100);
        assert_eq!(concat_usize([1, 0, 1].into_iter()), 101);
        assert_eq!(concat_usize([123, 45, 6789].into_iter()), 123456789);
    }

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&example_input()), 288);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&example_input()), 71503);
    }
}
