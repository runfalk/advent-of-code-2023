use anyhow::{anyhow, Result};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::path::Path;
use std::str::FromStr;

type Coordinate = (usize, usize);

struct Map {
    width: usize,
    height: usize,
    blocks: HashMap<Coordinate, usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Moves {
    estimated_cost: usize,
    current_cost: usize,
    x: usize,
    y: usize,
    dir: Dir,
    num_straight_moves: usize,
}

fn manhattan_distance((ax, ay): Coordinate, (bx, by): Coordinate) -> usize {
    ax.abs_diff(bx) + ay.abs_diff(by)
}

impl Dir {
    fn turn_left(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

impl Map {
    fn cheapest_path(&self, min_straight_moves: usize, max_straight_moves: usize) -> Option<usize> {
        let source = (0, 0);
        let target = (self.width - 1, self.height - 1);

        let mut to_visit = BinaryHeap::new();
        let mut visited = HashSet::new();

        to_visit.push(Reverse(Moves {
            estimated_cost: manhattan_distance(source, target),
            current_cost: 0,
            x: source.0,
            y: source.1,
            dir: Dir::Right,
            num_straight_moves: 0,
        }));
        visited.insert((source.0, source.1, Dir::Right, 0));
        to_visit.push(Reverse(Moves {
            estimated_cost: manhattan_distance(source, target),
            current_cost: 0,
            x: source.0,
            y: source.1,
            dir: Dir::Down,
            num_straight_moves: 0,
        }));
        visited.insert((source.0, source.1, Dir::Down, 0));

        while let Some(Reverse(mov)) = to_visit.pop() {
            if (mov.x, mov.y) == target {
                return Some(mov.current_cost);
            }

            for dir in [mov.dir, mov.dir.turn_left(), mov.dir.turn_right()] {
                let num_straight_moves = if dir == mov.dir {
                    mov.num_straight_moves + 1
                } else {
                    1
                };

                if num_straight_moves > max_straight_moves {
                    continue;
                }

                if dir != mov.dir && mov.num_straight_moves < min_straight_moves {
                    continue;
                }

                let Some((x, y)) = (match dir {
                    Dir::Up => mov.y.checked_sub(1).map(|y| (mov.x, y)),
                    Dir::Down => Some((mov.x, mov.y + 1)),
                    Dir::Left => mov.x.checked_sub(1).map(|x| (x, mov.y)),
                    Dir::Right => Some((mov.x + 1, mov.y)),
                }) else {
                    continue;
                };

                let Some(cost) = self.blocks.get(&(x, y)) else {
                    continue;
                };

                if !visited.insert((x, y, dir, num_straight_moves)) {
                    continue;
                }

                let new_move = Moves {
                    estimated_cost: mov.current_cost + cost + manhattan_distance((x, y), target),
                    current_cost: mov.current_cost + cost,
                    x,
                    y,
                    dir,
                    num_straight_moves,
                };
                to_visit.push(Reverse(new_move));
            }
        }
        None
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut blocks = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(cost) = c.to_digit(10) {
                    blocks.insert((x, y), usize::try_from(cost)?);
                } else {
                    return Err(anyhow!("Invalid character {:?}", c));
                }
                width = x + 1
            }
            height = y + 1
        }
        Ok(Self {
            width,
            height,
            blocks,
        })
    }
}

fn part_a(map: &Map) -> usize {
    map.cheapest_path(1, 3).unwrap()
}

fn part_b(map: &Map) -> usize {
    map.cheapest_path(4, 10).unwrap()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let map_str = std::fs::read_to_string(path)?;
    let map = map_str.parse()?;
    Ok((part_a(&map), part_b(&map).into()))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(17, 1256, 1382);

    const EXAMPLE_INPUT: &'static str = concat!(
        "2413432311323\n",
        "3215453535623\n",
        "3255245654254\n",
        "3446585845452\n",
        "4546657867536\n",
        "1438598798454\n",
        "4457876987766\n",
        "3637877979653\n",
        "4654967986887\n",
        "4564679986453\n",
        "1224686865563\n",
        "2546548887735\n",
        "4322674655533\n",
    );

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&EXAMPLE_INPUT.parse().unwrap()), 102);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&EXAMPLE_INPUT.parse().unwrap()), 94);
    }
}
