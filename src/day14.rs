use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::path::Path;

#[derive(Clone)]
struct Platform {
    width: usize,
    height: usize,
    round: HashSet<(usize, usize)>,
    cube: HashSet<(usize, usize)>,
}

impl Platform {
    fn load(&self) -> usize {
        self.round.iter().map(|(_, y)| self.height - y).sum()
    }

    fn tilt_north(&mut self) {
        // Start from the top
        for y in 1..self.height {
            for x in 0..self.width {
                let Some(_) = self.round.take(&(x, y)) else {
                    continue;
                };
                let mut ny = y;
                while ny > 0
                    && !self.round.contains(&(x, ny - 1))
                    && !self.cube.contains(&(x, ny - 1))
                {
                    ny -= 1;
                }
                self.round.insert((x, ny));
            }
        }
    }

    fn tilt_cycle(&mut self) {
        self.tilt_north();

        // West
        for x in 1..self.width {
            for y in 0..self.height {
                let Some(_) = self.round.take(&(x, y)) else {
                    continue;
                };
                let mut nx = x;
                while nx > 0
                    && !self.round.contains(&(nx - 1, y))
                    && !self.cube.contains(&(nx - 1, y))
                {
                    nx -= 1;
                }
                self.round.insert((nx, y));
            }
        }
        // South
        for y in (0..self.height - 1).rev() {
            for x in 0..self.width {
                let Some(_) = self.round.take(&(x, y)) else {
                    continue;
                };
                let mut ny = y;
                while ny + 1 < self.height
                    && !self.round.contains(&(x, ny + 1))
                    && !self.cube.contains(&(x, ny + 1))
                {
                    ny += 1;
                }
                self.round.insert((x, ny));
            }
        }
        // East
        for x in (0..self.width - 1).rev() {
            for y in 0..self.height {
                let Some(_) = self.round.take(&(x, y)) else {
                    continue;
                };
                let mut nx = x;
                while nx + 1 < self.width
                    && !self.round.contains(&(nx + 1, y))
                    && !self.cube.contains(&(nx + 1, y))
                {
                    nx += 1;
                }
                self.round.insert((nx, y));
            }
        }
    }
}

fn parse(s: &str) -> Result<Platform> {
    let mut round = HashSet::new();
    let mut cube = HashSet::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'O' => {
                    round.insert((x, y));
                }
                '#' => {
                    cube.insert((x, y));
                }
                '.' => {}
                _ => {
                    return Err(anyhow!("HM"));
                }
            }
            width = x + 1;
        }
        height = y + 1;
    }
    Ok(Platform {
        width,
        height,
        round,
        cube,
    })
}

fn part_a(mut platform: Platform) -> usize {
    platform.tilt_north();
    platform.load()
}

fn part_b(mut platform: Platform) -> usize {
    let mut history = Vec::new();
    let offset = loop {
        platform.tilt_cycle();
        if let Some(i) = history.iter().position(|r| r == &platform.round) {
            break i;
        }
        history.push(platform.round.clone());
    };
    let i = offset + (1_000_000_000 - offset - 1) % (history.len() - offset);
    platform.round = history[i].clone();
    platform.load()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input = std::fs::read_to_string(path)?;
    let platform = parse(&input)?;
    Ok((part_a(platform.clone()), part_b(platform.clone()).into()))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(14, 108918, 100310);

    const EXAMPLE_INPUT: &'static str = concat!(
        "OOOO.#.O..\n",
        "OO..#....#\n",
        "OO..O##..O\n",
        "O..#.OO...\n",
        "........#.\n",
        "..#....#.#\n",
        "..O..#.O.O\n",
        "..O.......\n",
        "#....###..\n",
        "#....#....\n",
    );

    const EXAMPLE_INPUT_1_CYCLE: &'static str = concat!(
        ".....#....\n",
        "....#...O#\n",
        "...OO##...\n",
        ".OO#......\n",
        ".....OOO#.\n",
        ".O#...O#.#\n",
        "....O#....\n",
        "......OOOO\n",
        "#...O###..\n",
        "#..OO#....\n",
    );

    const EXAMPLE_INPUT_2_CYCLE: &'static str = concat!(
        ".....#....\n",
        "....#...O#\n",
        ".....##...\n",
        "..O#......\n",
        ".....OOO#.\n",
        ".O#...O#.#\n",
        "....O#...O\n",
        ".......OOO\n",
        "#..OO###..\n",
        "#.OOO#...O\n",
    );

    const EXAMPLE_INPUT_3_CYCLE: &'static str = concat!(
        ".....#....\n",
        "....#...O#\n",
        ".....##...\n",
        "..O#......\n",
        ".....OOO#.\n",
        ".O#...O#.#\n",
        "....O#...O\n",
        ".......OOO\n",
        "#...O###.O\n",
        "#.OOO#...O\n",
    );

    #[test]
    fn test_tilt_cycle() {
        let mut start = parse(EXAMPLE_INPUT).unwrap();

        let after_1_cycle = parse(EXAMPLE_INPUT_1_CYCLE).unwrap();
        start.tilt_cycle();
        assert_eq!(start.round, after_1_cycle.round);

        let after_2_cycle = parse(EXAMPLE_INPUT_2_CYCLE).unwrap();
        start.tilt_cycle();
        assert_eq!(start.round, after_2_cycle.round);

        let after_3_cycle = parse(EXAMPLE_INPUT_3_CYCLE).unwrap();
        start.tilt_cycle();
        assert_eq!(start.round, after_3_cycle.round);
    }
}
