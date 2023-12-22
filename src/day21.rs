use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::str::FromStr;

struct Map {
    width: isize,
    height: isize,
    start: (isize, isize),
    walls: HashSet<(isize, isize)>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut start = None;
        let mut walls = HashSet::new();
        for (y, line) in s.lines().enumerate() {
            let y: isize = y.try_into()?;
            for (x, c) in line.chars().enumerate() {
                let x: isize = x.try_into()?;
                match c {
                    '#' => {
                        walls.insert((x, y));
                    }
                    '.' => {}
                    'S' => {
                        if start.is_some() {
                            return Err(anyhow!("Start is defined twice"));
                        }
                        start = Some((x, y));
                    }
                    _ => return Err(anyhow!("Invalid character in map {:?}", c)),
                }
                width = x + 1;
            }
            height = y + 1;
        }

        Ok(Self {
            width,
            height,
            start: start.ok_or_else(|| anyhow!("No starting position found"))?,
            walls,
        })
    }
}

impl Map {
    fn num_reachable_gardens(&self, step_limit: usize, infinite: bool) -> usize {
        let mut visited = HashMap::new();
        visited.insert(self.start, 0);

        let mut to_visit: VecDeque<((isize, isize), usize)> = VecDeque::new();
        to_visit.push_back((self.start, 0));

        while let Some(((x, y), steps)) = to_visit.pop_front() {
            if steps >= step_limit {
                continue;
            }
            let neighbors = if infinite {
                [
                    Some((x, y - 1)), // Up
                    Some((x, y + 1)), // Down
                    Some((x - 1, y)), // Left
                    Some((x + 1, y)), // Right
                ]
            } else {
                [
                    (y > 0).then_some((x, y - 1)),               // Up
                    (y + 1 < self.height).then_some((x, y + 1)), // Down
                    (x > 0).then_some((x - 1, y)),               // Left
                    (x + 1 < self.width).then_some((x + 1, y)),  // Right
                ]
            };
            for n in neighbors.into_iter().flatten() {
                let wrapped_n = (n.0.rem_euclid(self.width), n.1.rem_euclid(self.height));
                if self.walls.contains(&wrapped_n) || visited.contains_key(&n) {
                    continue;
                }
                visited.insert(n, steps + 1);
                to_visit.push_back((n, steps + 1));
            }
        }
        visited
            .into_values()
            .filter(|&steps| steps % 2 == step_limit % 2)
            .count()
    }
}

fn part_a(map: &Map) -> usize {
    map.num_reachable_gardens(64, false)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let map = std::fs::read_to_string(path)?.parse()?;
    Ok((part_a(&map), None))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(21, 3615);

    const EXAMPLE_INPUT: &'static str = concat!(
        "...........\n",
        ".....###.#.\n",
        ".###.##..#.\n",
        "..#.#...#..\n",
        "....#.#....\n",
        ".##..S####.\n",
        ".##..#...#.\n",
        ".......##..\n",
        ".##.#.####.\n",
        ".##..##.##.\n",
        "...........\n",
    );

    #[test]
    fn test_examples() {
        let map: Map = EXAMPLE_INPUT.parse().unwrap();
        assert_eq!(map.num_reachable_gardens(6, false), 16);
        assert_eq!(map.num_reachable_gardens(6, true), 16);
        assert_eq!(map.num_reachable_gardens(10, true), 50);
        assert_eq!(map.num_reachable_gardens(50, true), 1594);
        assert_eq!(map.num_reachable_gardens(100, true), 6536);
        assert_eq!(map.num_reachable_gardens(500, true), 167_004);
        assert_eq!(map.num_reachable_gardens(1000, true), 668_697);
    }
}
