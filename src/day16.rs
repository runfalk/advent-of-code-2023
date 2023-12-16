use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str::FromStr;

type Coordinate = (usize, usize);

struct Map {
    width: usize,
    height: usize,
    mirrors: HashMap<Coordinate, Mirror>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    x: usize,
    y: usize,
    dir: Dir,
}

enum Mirror {
    SplitUpDown,
    SplitLeftRight,
    ReflectBackslash,
    ReflectSlash,
}

enum MaybePair<T> {
    Pair(T, T),
    Single(T), // T_T
}

impl Beam {
    fn new(x: usize, y: usize, dir: Dir) -> Self {
        Self { x, y, dir }
    }

    fn turn(&self, dir: Dir) -> Self {
        Self { dir, ..*self }
    }
}

impl Mirror {
    fn reflect(&self, incoming_direction: Dir) -> MaybePair<Dir> {
        match (self, incoming_direction) {
            (Self::SplitUpDown, Dir::Left) | (Self::SplitUpDown, Dir::Right) => {
                MaybePair::Pair(Dir::Up, Dir::Down)
            }
            (Self::SplitUpDown, Dir::Up) => MaybePair::Single(Dir::Up),
            (Self::SplitUpDown, Dir::Down) => MaybePair::Single(Dir::Down),
            (Self::SplitLeftRight, Dir::Up) | (Self::SplitLeftRight, Dir::Down) => {
                MaybePair::Pair(Dir::Left, Dir::Right)
            }
            (Self::SplitLeftRight, Dir::Left) => MaybePair::Single(Dir::Left),
            (Self::SplitLeftRight, Dir::Right) => MaybePair::Single(Dir::Right),
            (Self::ReflectBackslash, Dir::Up) => MaybePair::Single(Dir::Left),
            (Self::ReflectBackslash, Dir::Down) => MaybePair::Single(Dir::Right),
            (Self::ReflectBackslash, Dir::Left) => MaybePair::Single(Dir::Up),
            (Self::ReflectBackslash, Dir::Right) => MaybePair::Single(Dir::Down),
            (Self::ReflectSlash, Dir::Up) => MaybePair::Single(Dir::Right),
            (Self::ReflectSlash, Dir::Down) => MaybePair::Single(Dir::Left),
            (Self::ReflectSlash, Dir::Left) => MaybePair::Single(Dir::Down),
            (Self::ReflectSlash, Dir::Right) => MaybePair::Single(Dir::Up),
        }
    }
}

impl<T> MaybePair<T> {
    fn map<U>(self, f: impl Fn(T) -> U) -> MaybePair<U> {
        match self {
            Self::Pair(a, b) => MaybePair::Pair(f(a), f(b)),
            Self::Single(v) => MaybePair::Single(f(v)),
        }
    }
}

impl Map {
    fn advance_beam(&self, beam: &Beam) -> Option<Beam> {
        let (x, y) = match beam.dir {
            Dir::Up => (beam.x, beam.y.checked_sub(1)?),
            Dir::Down => (beam.x, (beam.y + 1 < self.height).then_some(beam.y + 1)?),
            Dir::Left => (beam.x.checked_sub(1)?, beam.y),
            Dir::Right => ((beam.x + 1 < self.width).then_some(beam.x + 1)?, beam.y),
        };
        Some(Beam { x, y, ..*beam })
    }

    fn num_illuminated_tiles(&self, seed_beam: Beam) -> usize {
        let mut beams = vec![seed_beam];
        let mut visited = HashSet::new();
        while let Some(beam) = beams.pop() {
            // If we have already reached this tile from this angle we skip searching it
            if !visited.insert(beam) {
                continue;
            }

            let next_beam = self
                .mirrors
                .get(&(beam.x, beam.y))
                // We hit a mirror and need to consider the reflection
                .map(|mirror| {
                    mirror
                        .reflect(beam.dir)
                        .map(|d| self.advance_beam(&beam.turn(d)))
                })
                // We didn't hit a mirror and need to advance
                .unwrap_or_else(|| MaybePair::Single(self.advance_beam(&beam)));
            match next_beam {
                MaybePair::Pair(Some(a), Some(b)) => {
                    beams.push(a);
                    beams.push(b);
                }
                MaybePair::Pair(Some(v), None)
                | MaybePair::Pair(None, Some(v))
                | MaybePair::Single(Some(v)) => {
                    beams.push(v);
                }
                MaybePair::Pair(None, None) | MaybePair::Single(None) => {}
            }
        }

        // We shouldn't double count the same tile visited from different angles
        let unique_tiles: HashSet<_> = visited.into_iter().map(|b| (b.x, b.y)).collect();
        unique_tiles.len()
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut mirrors = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '/' => {
                        mirrors.insert((x, y), Mirror::ReflectSlash);
                    }
                    '\\' => {
                        mirrors.insert((x, y), Mirror::ReflectBackslash);
                    }
                    '|' => {
                        mirrors.insert((x, y), Mirror::SplitUpDown);
                    }
                    '-' => {
                        mirrors.insert((x, y), Mirror::SplitLeftRight);
                    }
                    '.' => {}
                    _ => return Err(anyhow!("Unknown tile {:?}", c)),
                }
                width = x + 1
            }
            height = y + 1
        }
        Ok(Self {
            width,
            height,
            mirrors,
        })
    }
}

fn part_a(map: &Map) -> usize {
    map.num_illuminated_tiles(Beam::new(0, 0, Dir::Right))
}

fn part_b(map: &Map) -> usize {
    let from_top = (0..map.width).map(|x| Beam::new(x, 0, Dir::Down));
    let from_bottom = (0..map.width).map(|x| Beam::new(x, map.height - 1, Dir::Up));
    let from_left = (0..map.height).map(|y| Beam::new(0, y, Dir::Right));
    let from_right = (0..map.height).map(|y| Beam::new(map.width - 1, y, Dir::Left));
    from_top
        .chain(from_bottom)
        .chain(from_left)
        .chain(from_right)
        .map(|seed_beam| map.num_illuminated_tiles(seed_beam))
        .max()
        .unwrap()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let map_str = std::fs::read_to_string(path)?;
    let map = map_str.parse()?;
    Ok((part_a(&map), part_b(&map).into()))
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &'static str = concat!(
        ".|...\\....\n",
        "|.-.\\.....\n",
        ".....|-...\n",
        "........|.\n",
        "..........\n",
        ".........\\\n",
        "..../.\\\\..\n",
        ".-.-/..|..\n",
        ".|....-|.\\\n",
        "..//.|....\n",
    );

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&EXAMPLE_INPUT.parse().unwrap()), 46);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&EXAMPLE_INPUT.parse().unwrap()), 51);
    }
}
