use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    NorthSouth,
    WestEast,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Coordinate {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn translate(&self, dir: Direction) -> Self {
        match dir {
            Direction::North => Self::new(self.x, self.y - 1),
            Direction::South => Self::new(self.x, self.y + 1),
            Direction::West => Self::new(self.x - 1, self.y),
            Direction::East => Self::new(self.x + 1, self.y),
        }
    }
}

impl Direction {
    fn iter() -> impl Iterator<Item = Self> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .into_iter()
    }
}

impl Pipe {
    fn exit(&self, move_direction: Direction) -> Option<Direction> {
        match (self, move_direction) {
            (Self::NorthSouth, Direction::South) => Some(Direction::South),
            (Self::NorthSouth, Direction::North) => Some(Direction::North),
            (Self::WestEast, Direction::East) => Some(Direction::East),
            (Self::WestEast, Direction::West) => Some(Direction::West),
            (Self::NorthWest, Direction::South) => Some(Direction::West),
            (Self::NorthWest, Direction::East) => Some(Direction::North),
            (Self::NorthEast, Direction::South) => Some(Direction::East),
            (Self::NorthEast, Direction::West) => Some(Direction::North),
            (Self::SouthWest, Direction::North) => Some(Direction::West),
            (Self::SouthWest, Direction::East) => Some(Direction::South),
            (Self::SouthEast, Direction::North) => Some(Direction::East),
            (Self::SouthEast, Direction::West) => Some(Direction::South),
            _ => None,
        }
    }
}

fn find_loop_path(start: Coordinate, pipes: &HashMap<Coordinate, Pipe>) -> Option<Vec<Coordinate>> {
    let mut move_direction = pipes.get(&start).map(|p| match p {
        Pipe::NorthSouth => Direction::South,
        Pipe::WestEast => Direction::East,
        Pipe::NorthWest => Direction::South,
        Pipe::NorthEast => Direction::South,
        Pipe::SouthWest => Direction::North,
        Pipe::SouthEast => Direction::North,
    })?;
    let mut pos = start;
    let mut path = Vec::new();
    while let Some(pipe) = pipes.get(&pos) {
        if path.contains(&pos) {
            return Some(path);
        }
        path.push(pos);

        let Some(new_move_direction) = pipe.exit(move_direction) else {
            break;
        };

        pos = pos.translate(new_move_direction);
        move_direction = new_move_direction;
    }
    None
}

fn part_a(start: Coordinate, pipes: &HashMap<Coordinate, Pipe>) -> Result<usize> {
    let path = find_loop_path(start, pipes).ok_or_else(|| anyhow!("Unable to find loop"))?;
    Ok(path.len() / 2)
}

fn part_b(start: Coordinate, pipes: &HashMap<Coordinate, Pipe>) -> Result<usize> {
    let path = find_loop_path(start, pipes).ok_or_else(|| anyhow!("Unable to find loop"))?;

    // Pipes that flip whether or not we are inside the enclosed loop
    let special_pipes = [Pipe::NorthSouth, Pipe::NorthEast, Pipe::NorthWest];

    let min_x = path.iter().map(|c| c.x).min().unwrap();
    let max_x = path.iter().map(|c| c.x).max().unwrap();
    let min_y = path.iter().map(|c| c.y).min().unwrap();
    let max_y = path.iter().map(|c| c.y).max().unwrap();

    let mut num_inside = 0;
    for y in min_y..=max_y {
        let mut is_inside = false;
        for x in min_x..=max_x {
            let c = Coordinate::new(x, y);
            if !path.contains(&c) {
                if is_inside {
                    num_inside += 1;
                }
                continue;
            }

            // Should be safe since a pipe should always be on the loop path
            let pipe = pipes.get(&c).copied().unwrap();
            if special_pipes.contains(&pipe) {
                is_inside = !is_inside;
            }
        }
    }
    Ok(num_inside)
}

fn parse_pipes(s: &str) -> Result<(Coordinate, HashMap<Coordinate, Pipe>)> {
    let mut start = None;
    let mut pipes = HashMap::new();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Coordinate::new(x as isize, y as isize);
            let pipe = match c {
                '|' => Pipe::NorthSouth,
                '-' => Pipe::WestEast,
                'L' => Pipe::NorthEast,
                'J' => Pipe::NorthWest,
                'F' => Pipe::SouthEast,
                '7' => Pipe::SouthWest,
                '.' => {
                    continue;
                }
                'S' => {
                    start = Some(p);
                    continue;
                }
                _ => {
                    return Err(anyhow!("Unknown tile {:?}", c));
                }
            };
            pipes.insert(p, pipe);
        }
    }

    // Determine type of the start pipe
    let start = start.ok_or_else(|| anyhow!("No starting position found"))?;
    for start_pipe in [
        Pipe::NorthSouth,
        Pipe::WestEast,
        Pipe::NorthWest,
        Pipe::NorthEast,
        Pipe::SouthWest,
        Pipe::SouthEast,
    ] {
        let is_valid_start_pipe = Direction::iter()
            .filter_map(|d| start_pipe.exit(d))
            .all(|d| {
                pipes
                    .get(&start.translate(d))
                    .and_then(|p| p.exit(d))
                    .is_some()
            });
        if is_valid_start_pipe {
            pipes.insert(start, start_pipe);
            return Ok((start, pipes));
        }
    }
    Err(anyhow!(
        "Failed to determine pipe type for the starting position"
    ))
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = std::fs::read_to_string(path)?;
    let (start, pipes) = parse_pipes(&file)?;
    Ok((part_a(start, &pipes)?, part_b(start, &pipes)?.into()))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(10, 6757, 523);

    #[rustfmt::skip]
    const EXAMPLE_1_A: &'static str = concat!(
        ".....\n",
        ".S-7.\n",
        ".|.|.\n",
        ".L-J.\n",
        ".....\n",
    );

    #[rustfmt::skip]
    const EXAMPLE_1_B: &'static str = concat!(
        "..F7.\n",
        ".FJ|.\n",
        "SJ.L7\n",
        "|F--J\n",
        "LJ...\n",
    );

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
