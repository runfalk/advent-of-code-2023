use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cube {
    a: Coord,
    b: Coord,
}

#[derive(Debug, Default)]
struct SupportInfo {
    supports: HashSet<usize>,
    supported_by: HashSet<usize>,
}

impl Cube {
    fn iter_xy_plane(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (self.a.y..=self.b.y).flat_map(|y| (self.a.x..=self.b.x).map(move |x| (x, y)))
    }

    fn shares_xy_plane(&self, other: &Self) -> bool {
        self.iter_xy_plane()
            .any(|self_xy| other.iter_xy_plane().any(|other_xy| self_xy == other_xy))
    }

    fn with_z(&self, z: usize) -> Self {
        Self {
            a: Coord {
                x: self.a.x,
                y: self.a.y,
                z,
            },
            b: Coord {
                x: self.b.x,
                y: self.b.y,
                z: z + self.b.z - self.a.z,
            },
        }
    }
}

impl SupportInfo {
    fn support_graph(settled_cubes: &[Cube]) -> HashMap<usize, Self> {
        let mut graph: HashMap<usize, Self> = HashMap::new();
        for i in 0..settled_cubes.len() {
            // Make sure that we have an entry for every cube
            graph.entry(i).or_default();

            let cube = settled_cubes[i];
            for (j, below_cube) in settled_cubes.iter().enumerate().take(i) {
                if below_cube.b.z + 1 != cube.a.z || !cube.shares_xy_plane(below_cube) {
                    continue;
                }
                graph.entry(i).or_default().supported_by.insert(j);
                graph.entry(j).or_default().supports.insert(i);
            }
        }
        graph
    }
}

impl FromStr for Cube {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((a_str, b_str)) = s.split_once('~') else {
            return Err(anyhow!("Missing '~' in cube specification"));
        };
        Ok(Self {
            a: a_str.parse()?,
            b: b_str.parse()?,
        })
    }
}

impl FromStr for Coord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((x_str, rest)) = s.split_once(',') else {
            return Err(anyhow!("Missing ',' in cube specification"));
        };
        let Some((y_str, z_str)) = rest.split_once(',') else {
            return Err(anyhow!("Missing ',' in cube specification"));
        };
        Ok(Self {
            x: x_str.parse()?,
            y: y_str.parse()?,
            z: z_str.parse()?,
        })
    }
}

fn settle(falling_cubes: &[Cube]) -> Vec<Cube> {
    let mut sorted_cubes = falling_cubes.to_vec();
    sorted_cubes.sort_by_key(|c| (c.a.z, c.b.z));

    for i in 0..sorted_cubes.len() {
        let mut supported_at = 0;
        let cube = sorted_cubes[i];
        for prev_cube in sorted_cubes.iter().take(i) {
            if cube.shares_xy_plane(prev_cube) {
                supported_at = supported_at.max(prev_cube.b.z);
            }
        }
        sorted_cubes[i] = cube.with_z(supported_at + 1);
    }

    sorted_cubes.sort_by_key(|c| (c.a.z, c.b.z));
    sorted_cubes
}

fn part_a(falling_cubes: &[Cube]) -> usize {
    let settled_cubes = settle(falling_cubes);
    let support_info = SupportInfo::support_graph(&settled_cubes);

    let mut num_removable = 0;
    for removed_cube_info in support_info.values() {
        let mut can_remove = true;
        for supported_cube in removed_cube_info.supports.iter() {
            let supported_cube_info = &support_info[supported_cube];
            if supported_cube_info.supported_by.len() < 2 {
                can_remove = false;
                break;
            }
        }
        if can_remove {
            num_removable += 1;
        }
    }
    num_removable
}

fn part_b(falling_cubes: &[Cube]) -> usize {
    let settled_cubes = settle(falling_cubes);
    let support_info = SupportInfo::support_graph(&settled_cubes);

    let mut num_chains = 0;
    for cube_to_remove in 0..settled_cubes.len() {
        let mut to_visit = VecDeque::new();
        let mut visited = HashSet::new();
        to_visit.push_back(cube_to_remove);
        visited.insert(cube_to_remove);

        while let Some(removed_cube) = to_visit.pop_front() {
            let removed_cube_info = &support_info[&removed_cube];
            for supported_cube in removed_cube_info.supports.iter().copied() {
                let supported_cube_info = &support_info[&supported_cube];
                if supported_cube_info
                    .supported_by
                    .difference(&visited)
                    .count()
                    == 0
                    && visited.insert(supported_cube)
                {
                    num_chains += 1;
                    to_visit.push_back(supported_cube);
                }
            }
        }
    }
    num_chains
}

fn parse_cubes(s: &str) -> Result<Vec<Cube>> {
    s.lines().map(|l| l.parse()).collect()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let cubes_str = std::fs::read_to_string(path)?;
    let cubes = parse_cubes(&cubes_str)?;
    Ok((part_a(&cubes), part_b(&cubes).into()))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(22, 407, 59266);

    const EXAMPLE_INPUT: &'static str = concat!(
        "1,0,1~1,2,1\n",
        "0,0,2~2,0,2\n",
        "0,2,3~2,2,3\n",
        "0,0,4~0,2,4\n",
        "2,0,5~2,2,5\n",
        "0,1,6~2,1,6\n",
        "1,1,8~1,1,9\n",
    );

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&parse_cubes(EXAMPLE_INPUT).unwrap()), 5);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&parse_cubes(EXAMPLE_INPUT).unwrap()), 7);
    }
}
