use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Map {
    width: usize,
    height: usize,
    forest: HashSet<(usize, usize)>,
    slopes: HashMap<(usize, usize), Direction>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut forest = HashSet::new();
        let mut slopes = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let p = (x, y);
                match c {
                    '#' => {
                        forest.insert(p);
                    }
                    '^' => {
                        slopes.insert(p, Direction::Up);
                    }
                    'v' => {
                        slopes.insert(p, Direction::Down);
                    }
                    '<' => {
                        slopes.insert(p, Direction::Left);
                    }
                    '>' => {
                        slopes.insert(p, Direction::Right);
                    }
                    '.' => {}
                    _ => return Err(anyhow!("Unknown character in map {:?}", c)),
                }
                width = x + 1;
            }
            height = y + 1;
        }
        Ok(Self {
            width,
            height,
            forest,
            slopes,
        })
    }
}

fn part_a(map: &Map) -> usize {
    #[allow(clippy::type_complexity)]
    let mut to_visit: VecDeque<(usize, usize, HashSet<(usize, usize)>)> = VecDeque::new();
    to_visit.push_back((1, 0, [(1, 0)].into_iter().collect()));

    let mut longest_path = 0;
    while let Some((x, y, visited)) = to_visit.pop_front() {
        if (x, y) == (map.width - 2, map.height - 1) {
            // -1 because our visited set includes the starting tile
            longest_path = longest_path.max(visited.len() - 1);
            continue;
        }

        let neighbors = [
            y.checked_sub(1).map(|ny| (Direction::Up, x, ny)),
            (y + 1 < map.height).then_some((Direction::Down, x, y + 1)),
            x.checked_sub(1).map(|nx| (Direction::Left, nx, y)),
            (x + 1 < map.width).then_some((Direction::Right, x + 1, y)),
        ];
        for (nd, nx, ny) in neighbors.into_iter().flatten() {
            let n = (nx, ny);
            if map.forest.contains(&n)
                || map.slopes.get(&n).map(|&sd| sd != nd).unwrap_or(false)
                || visited.contains(&n)
            {
                continue;
            }
            let mut neighbor_visited = visited.clone();
            neighbor_visited.insert(n);
            to_visit.push_back((nx, ny, neighbor_visited));
        }
    }

    longest_path
}

fn part_b(map: &Map) -> usize {
    let source = (1usize, 0usize);
    let target = (map.width - 2, map.height - 1);

    // Find all branching points and construct a graph
    let mut to_visit = Vec::new();
    let mut visited = HashSet::new();
    to_visit.push((source.0, source.1, 0, source));
    visited.insert(target);

    let mut graph: HashMap<(usize, usize), HashMap<(usize, usize), usize>> = HashMap::new();
    while let Some((x, y, mut cost, mut branch_start)) = to_visit.pop() {
        let neighbors: Vec<(usize, usize)> = [
            y.checked_sub(1).map(|ny| (x, ny)),
            (y + 1 < map.height).then_some((x, y + 1)),
            x.checked_sub(1).map(|nx| (nx, y)),
            (x + 1 < map.width).then_some((x + 1, y)),
        ]
        .into_iter()
        .flatten()
        .filter(|n| !map.forest.contains(n))
        .collect();

        if (neighbors.len() > 2 || (x, y) == target) && (x, y) != branch_start {
            let max_cost = graph
                .entry(branch_start)
                .or_default()
                .entry((x, y))
                .or_insert(0);
            if *max_cost < cost {
                *max_cost = cost;
                *graph
                    .entry((x, y))
                    .or_default()
                    .entry(branch_start)
                    .or_insert(0) = cost;
            }

            // Reset cost and branching points
            cost = 0;
            branch_start = (x, y);
        }

        if !visited.insert((x, y)) {
            continue;
        }

        for (nx, ny) in neighbors {
            to_visit.push((nx, ny, cost + 1, branch_start));
        }
    }

    // Find the most expensive path in the graph using brute force
    #[allow(clippy::type_complexity)]
    let mut to_visit: Vec<(usize, usize, usize, HashSet<(usize, usize)>)> = Vec::new();
    to_visit.push((source.0, source.1, 0, HashSet::new()));

    let mut max_cost = 0;
    while let Some((x, y, acc_cost, mut visited)) = to_visit.pop() {
        if !visited.insert((x, y)) {
            continue;
        }
        if (x, y) == target {
            max_cost = max_cost.max(acc_cost);
            continue;
        }
        for (n, cost) in graph.get(&(x, y)).unwrap().iter() {
            to_visit.push((n.0, n.1, acc_cost + cost, visited.clone()));
        }
    }
    max_cost
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let map: Map = std::fs::read_to_string(path)?.parse()?;
    Ok((part_a(&map), part_b(&map).into()))
}

#[cfg(test)]
mod test {
    use super::*;

    // Takes 15 seconds to run :(
    test_real_input!(
        #[ignore]
        23,
        2202,
        6226
    );

    const EXAMPLE_INPUT: &'static str = concat!(
        "#.#####################\n",
        "#.......#########...###\n",
        "#######.#########.#.###\n",
        "###.....#.>.>.###.#.###\n",
        "###v#####.#v#.###.#.###\n",
        "###.>...#.#.#.....#...#\n",
        "###v###.#.#.#########.#\n",
        "###...#.#.#.......#...#\n",
        "#####.#.#.#######.#.###\n",
        "#.....#.#.#.......#...#\n",
        "#.#####.#.#.#########v#\n",
        "#.#...#...#...###...>.#\n",
        "#.#.#v#######v###.###v#\n",
        "#...#.>.#...>.>.#.###.#\n",
        "#####v#.#.###v#.#.###.#\n",
        "#.....#...#...#.#.#...#\n",
        "#.#########.###.#.#.###\n",
        "#...###...#...#...#.###\n",
        "###.###.#.###v#####v###\n",
        "#...#...#.#.>.>.#.>.###\n",
        "#.###.###.#.###.#.#v###\n",
        "#.....###...###...#...#\n",
        "#####################.#\n",
    );

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&EXAMPLE_INPUT.parse().unwrap()), 94);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&EXAMPLE_INPUT.parse().unwrap()), 154);
    }
}
