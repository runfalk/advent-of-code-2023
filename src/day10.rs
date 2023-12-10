use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
enum Pipe {
    NS,
    WE,
    NE,
    NW,
    SE,
    SW,
}

impl Pipe {
    fn exit(self, enter_from: (isize, isize), pipe_pos: (isize, isize)) -> Option<(isize, isize)> {
        match (self, enter_from.0 - pipe_pos.0, enter_from.1 - pipe_pos.1) {
            (Self::NS, 0, -1) => Some((pipe_pos.0, pipe_pos.1 + 1)),
            (Self::NS, 0, 1) => Some((pipe_pos.0, pipe_pos.1 - 1)),
            (Self::WE, -1, 0) => Some((pipe_pos.0 + 1, pipe_pos.1)),
            (Self::WE, 1, 0) => Some((pipe_pos.0 - 1, pipe_pos.1)),
            (Self::NE, 0, -1) => Some((pipe_pos.0 + 1, pipe_pos.1)),
            (Self::NE, 1, 0) => Some((pipe_pos.0, pipe_pos.1 - 1)),
            (Self::NW, 0, -1) => Some((pipe_pos.0 - 1, pipe_pos.1)),
            (Self::NW, -1, 0) => Some((pipe_pos.0, pipe_pos.1 - 1)),
            (Self::SE, 0, 1) => Some((pipe_pos.0 + 1, pipe_pos.1)),
            (Self::SE, 1, 0) => Some((pipe_pos.0, pipe_pos.1 + 1)),
            (Self::SW, 0, 1) => Some((pipe_pos.0 - 1, pipe_pos.1)),
            (Self::SW, -1, 0) => Some((pipe_pos.0, pipe_pos.1 + 1)),
            _ => None,
        }
    }
}

fn find_loop_path(
    start: (isize, isize),
    pipes: &HashMap<(isize, isize), Pipe>,
) -> Option<Vec<(isize, isize)>> {
    let mut pipes = pipes.clone();
    for (start_pipe, enter_dx, enter_dy) in [
        (Pipe::NS, 0, -1),
        (Pipe::WE, -1, 0),
        (Pipe::NE, 0, -1),
        (Pipe::NW, 0, -1),
        (Pipe::SE, 0, 1),
        (Pipe::SW, 0, 1),
    ] {
        pipes.insert(start, start_pipe);

        let mut enter_from = (start.0 + enter_dx, start.1 + enter_dy);
        let mut pipe_pos = start;
        let mut path = Vec::new();
        while let Some(pipe) = pipes.get(&pipe_pos) {
            if path.contains(&pipe_pos) {
                return Some(path);
            }
            path.push(pipe_pos);

            let Some(exit_to) = pipe.exit(enter_from, pipe_pos) else {
                break;
            };

            enter_from = pipe_pos;
            pipe_pos = exit_to;
        }
    }
    None
}

fn part_a(start: (isize, isize), pipes: &HashMap<(isize, isize), Pipe>) -> Result<usize> {
    let path = find_loop_path(start, pipes).ok_or_else(|| anyhow!("Unable to find loop"))?;
    Ok(path.len() / 2)
}

fn part_b(start: (isize, isize), pipes: &HashMap<(isize, isize), Pipe>) -> Result<usize> {
    let path = find_loop_path(start, pipes).ok_or_else(|| anyhow!("Unable to find loop"))?;

    let mut left_of = HashSet::new();
    let mut right_of = HashSet::new();
    let iter_triplets = path
        .iter()
        .cycle()
        .take(path.len() + 1)
        .copied()
        .zip(path.iter().cycle().skip(1).copied())
        .zip(path.iter().cycle().skip(2).copied());
    for ((enter_from, pipe_pos), exit_to) in iter_triplets {
        let (enter_dx, enter_dy) = (enter_from.0 - pipe_pos.0, enter_from.1 - pipe_pos.1);
        let (exit_dx, exit_dy) = (exit_to.0 - pipe_pos.0, exit_to.1 - pipe_pos.1);
        match ((enter_dx, enter_dy), (exit_dx, exit_dy)) {
            // North to south
            ((0, -1), (0, 1)) => {
                left_of.insert((pipe_pos.0 + 1, pipe_pos.1));
                right_of.insert((pipe_pos.0 - 1, pipe_pos.1));
            }
            // South to north
            ((0, 1), (0, -1)) => {
                left_of.insert((pipe_pos.0 - 1, pipe_pos.1));
                right_of.insert((pipe_pos.0 + 1, pipe_pos.1));
            }
            // West to east
            ((-1, 0), (1, 0)) => {
                left_of.insert((pipe_pos.0, pipe_pos.1 - 1));
                right_of.insert((pipe_pos.0, pipe_pos.1 + 1));
            }
            // East to west
            ((1, 0), (-1, 0)) => {
                left_of.insert((pipe_pos.0, pipe_pos.1 + 1));
                right_of.insert((pipe_pos.0, pipe_pos.1 - 1));
            }
            // North to west
            ((0, -1), (-1, 0)) => {
                left_of.insert((pipe_pos.0 + 1, pipe_pos.1));
                left_of.insert((pipe_pos.0 + 1, pipe_pos.1 + 1));
                left_of.insert((pipe_pos.0, pipe_pos.1 + 1));
            }
            // West to north
            ((-1, 0), (0, -1)) => {
                right_of.insert((pipe_pos.0 + 1, pipe_pos.1));
                right_of.insert((pipe_pos.0 + 1, pipe_pos.1 + 1));
                right_of.insert((pipe_pos.0, pipe_pos.1 + 1));
            }
            // North to east
            ((0, -1), (1, 0)) => {
                right_of.insert((pipe_pos.0 - 1, pipe_pos.1));
                right_of.insert((pipe_pos.0 - 1, pipe_pos.1 + 1));
                right_of.insert((pipe_pos.0, pipe_pos.1 + 1));
            }
            // East to north
            ((1, 0), (0, -1)) => {
                left_of.insert((pipe_pos.0 - 1, pipe_pos.1));
                left_of.insert((pipe_pos.0 - 1, pipe_pos.1 + 1));
                left_of.insert((pipe_pos.0, pipe_pos.1 + 1));
            }
            // South to west
            ((0, 1), (-1, 0)) => {
                right_of.insert((pipe_pos.0 + 1, pipe_pos.1));
                right_of.insert((pipe_pos.0 + 1, pipe_pos.1 - 1));
                right_of.insert((pipe_pos.0, pipe_pos.1 - 1));
            }
            // West to south
            ((-1, 0), (0, 1)) => {
                left_of.insert((pipe_pos.0 + 1, pipe_pos.1));
                left_of.insert((pipe_pos.0 + 1, pipe_pos.1 - 1));
                left_of.insert((pipe_pos.0, pipe_pos.1 - 1));
            }
            // South to east
            ((0, 1), (1, 0)) => {
                left_of.insert((pipe_pos.0 - 1, pipe_pos.1));
                left_of.insert((pipe_pos.0 - 1, pipe_pos.1 - 1));
                left_of.insert((pipe_pos.0, pipe_pos.1 - 1));
            }
            // East to south
            ((1, 0), (0, 1)) => {
                right_of.insert((pipe_pos.0 - 1, pipe_pos.1));
                right_of.insert((pipe_pos.0 - 1, pipe_pos.1 - 1));
                right_of.insert((pipe_pos.0, pipe_pos.1 - 1));
            }
            _ => unreachable!(),
        }
    }

    // Find the bounding box of all pipes
    let min_x = *path.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *path.iter().map(|(x, _)| x).max().unwrap();
    let min_y = *path.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *path.iter().map(|(_, y)| y).max().unwrap();

    // Construct a set of bordering cells so can figure out whether the left or right set is
    // outside of the loop
    let mut border = HashSet::new();
    border.extend((min_x - 1..=max_x + 1).map(move |x| (x, min_y - 1))); // Top
    border.extend((min_x - 1..=max_x + 1).map(move |x| (x, max_y + 1))); // Bottom
    border.extend((min_y - 1..=max_y + 1).map(move |y| (min_x - 1, y))); // Left
    border.extend((min_y - 1..=max_y + 1).map(move |y| (max_x + 1, y))); // Right

    let mut to_fill = if left_of.is_disjoint(&border) {
        left_of
    } else {
        right_of
    };

    // Perform flood fill
    to_fill.retain(|pos| !path.contains(pos));

    let mut to_visit = to_fill.into_iter().collect::<Vec<_>>();
    let mut enclosed_tiles = HashSet::new();
    while let Some(tile) = to_visit.pop() {
        if !enclosed_tiles.insert(tile) {
            continue;
        }
        for n in [
            (tile.0, tile.1 - 1),
            (tile.0 + 1, tile.1),
            (tile.0, tile.1 + 1),
            (tile.0 - 1, tile.1),
        ] {
            if path.contains(&n) {
                continue;
            }
            to_visit.push(n);
        }
    }

    Ok(enclosed_tiles.len())
}

#[allow(clippy::type_complexity)]
fn parse_pipes(s: &str) -> Result<((isize, isize), HashMap<(isize, isize), Pipe>)> {
    let mut start = None;
    let mut pipes = HashMap::new();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = (x as isize, y as isize);
            match c {
                '|' => {
                    pipes.insert(p, Pipe::NS);
                }
                '-' => {
                    pipes.insert(p, Pipe::WE);
                }
                'L' => {
                    pipes.insert(p, Pipe::NE);
                }
                'J' => {
                    pipes.insert(p, Pipe::NW);
                }
                'F' => {
                    pipes.insert(p, Pipe::SE);
                }
                '7' => {
                    pipes.insert(p, Pipe::SW);
                }
                '.' => {}
                'S' => {
                    start = Some(p);
                }
                _ => {
                    return Err(anyhow!("Unknown tile {:?}", c));
                }
            }
        }
    }

    let start = start.ok_or_else(|| anyhow!("No starting position found"))?;
    Ok((start, pipes))
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = std::fs::read_to_string(path)?;
    let (start, pipes) = parse_pipes(&file)?;
    Ok((part_a(start, &pipes)?, part_b(start, &pipes)?.into()))
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1_A: &'static str =
        concat!(".....\n", ".S-7.\n", ".|.|.\n", ".L-J.\n", ".....\n",);

    const EXAMPLE_1_B: &'static str =
        concat!("..F7.\n", ".FJ|.\n", "SJ.L7\n", "|F--J\n", "LJ...\n",);

    const EXAMPLE_2_A: &'static str = concat!(
        "...........\n",
        ".S-------7.\n",
        ".|F-----7|.\n",
        ".||.....||.\n",
        ".||.....||.\n",
        ".|L-7.F-J|.\n",
        ".|..|.|..|.\n",
        ".L--J.L--J.\n",
        "...........\n",
    );

    const EXAMPLE_2_B: &'static str = concat!(
        "..........\n",
        ".S------7.\n",
        ".|F----7|.\n",
        ".||....||.\n",
        ".||....||.\n",
        ".|L-7F-J|.\n",
        ".|..||..|.\n",
        ".L--JL--J.\n",
        "..........\n",
    );

    const EXAMPLE_2_C: &'static str = concat!(
        ".F----7F7F7F7F-7....\n",
        ".|F--7||||||||FJ....\n",
        ".||.FJ||||||||L7....\n",
        "FJL7L7LJLJ||LJ.L-7..\n",
        "L--J.L7...LJS7F-7L7.\n",
        "....F-J..F7FJ|L7L7L7\n",
        "....L7.F7||L7|.L7L7|\n",
        ".....|FJLJ|FJ|F7|.LJ\n",
        "....FJL-7.||.||||...\n",
        "....L---J.LJ.LJLJ...\n",
    );

    const EXAMPLE_2_D: &'static str = concat!(
        "FF7FSF7F7F7F7F7F---7\n",
        "L|LJ||||||||||||F--J\n",
        "FL-7LJLJ||||||LJL-77\n",
        "F--JF--7||LJLJ7F7FJ-\n",
        "L---JF-JLJ.||-FJLJJ7\n",
        "|F|F-JF---7F7-L7L|7|\n",
        "|FFJF7L7F-JF7|JL---7\n",
        "7-L-JL7||F7|L7F-7F7|\n",
        "L.L7LFJ|||||FJL7||LJ\n",
        "L7JLJL-JLJLJL--JLJ.L\n",
    );

    #[test]
    fn test_part_a() {
        {
            let (start, pipes) = parse_pipes(EXAMPLE_1_A).unwrap();
            assert_eq!(part_a(start, &pipes).unwrap(), 4);
        }
        {
            let (start, pipes) = parse_pipes(EXAMPLE_1_B).unwrap();
            assert_eq!(part_a(start, &pipes).unwrap(), 8);
        }
    }

    #[test]
    fn test_part_b() {
        {
            let (start, pipes) = parse_pipes(EXAMPLE_2_A).unwrap();
            assert_eq!(part_b(start, &pipes).unwrap(), 4);
        }
        {
            let (start, pipes) = parse_pipes(EXAMPLE_2_B).unwrap();
            assert_eq!(part_b(start, &pipes).unwrap(), 4);
        }
        {
            let (start, pipes) = parse_pipes(EXAMPLE_2_C).unwrap();
            assert_eq!(part_b(start, &pipes).unwrap(), 8);
        }
        {
            let (start, pipes) = parse_pipes(EXAMPLE_2_D).unwrap();
            assert_eq!(part_b(start, &pipes).unwrap(), 10);
        }
    }
}
