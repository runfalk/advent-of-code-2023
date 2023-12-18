use anyhow::{anyhow, Result};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Coordinate {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn advance(self, dir: Dir, steps: isize) -> Self {
        match dir {
            Dir::Up => Self::new(self.x, self.y - steps),
            Dir::Down => Self::new(self.x, self.y + steps),
            Dir::Left => Self::new(self.x - steps, self.y),
            Dir::Right => Self::new(self.x + steps, self.y),
        }
    }
}

#[allow(clippy::type_complexity)]
fn parse_instructions(s: &str) -> Result<(Vec<(Dir, usize)>, Vec<(Dir, usize)>)> {
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in s.lines() {
        // A
        let Some((dir_str, rest)) = line.split_once(' ') else {
            return Err(anyhow!("Invalid instruction {:?}", line));
        };
        let Some((num_str, rest)) = rest.split_once(' ') else {
            return Err(anyhow!("Invalid instruction {:?}", line));
        };

        let a_dir = match dir_str {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => return Err(anyhow!("Unexpected direction {:?}", dir_str)),
        };
        a.push((a_dir, num_str.parse()?));

        // B
        let Some(color_str) = rest
            .strip_prefix("(#")
            .and_then(|s| s.strip_suffix(')'))
            .filter(|s| s.len() == 6)
        else {
            return Err(anyhow!("Invalid color {:?}", rest));
        };
        let color = usize::from_str_radix(color_str, 16)?;
        let b_dir = match color & 0xf {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Up,
            _ => return Err(anyhow!("Invalid direction in color string {:?}", color_str)),
        };
        b.push((b_dir, color >> 4));
    }
    Ok((a, b))
}

fn trench_area(dig_instructions: &[(Dir, usize)]) -> usize {
    // We start digging at 0x0 to simplify the calculation
    let mut curr = Coordinate::new(0, 0);
    let mut trench_corners = vec![curr];

    // Find all corners of the trench as well as the number of cells on the edge
    let mut edge = 0;
    for (dir, num) in dig_instructions {
        curr = curr.advance(*dir, *num as isize);
        trench_corners.push(curr);
        edge += *num;
    }

    // Use a modified version of the shoelace formula (Gauss formula) to calculate the area. I
    // think this is not covering half of the edge cells which is why we need to add them at the
    // end.
    let mut sum = 0;
    for (a, b) in trench_corners
        .iter()
        .copied()
        .zip(trench_corners.iter().copied().skip(1))
    {
        // We can skip a term here because we start at 0x0
        sum += a.x * b.y - b.x * a.y;
    }

    // We add the edge tiles and add one to compensate for some reason
    (sum.unsigned_abs() + edge) / 2 + 1
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let instructions_str = std::fs::read_to_string(path)?;
    let (a_instructions, b_instructions) = parse_instructions(&instructions_str)?;
    Ok((
        trench_area(&a_instructions),
        trench_area(&b_instructions).into(),
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(18, 58_550, 47_452_118_468_566);

    const EXAMPLE_INPUT: &'static str = concat!(
        "R 6 (#70c710)\n",
        "D 5 (#0dc571)\n",
        "L 2 (#5713f0)\n",
        "D 2 (#d2c081)\n",
        "R 2 (#59c680)\n",
        "D 2 (#411b91)\n",
        "L 5 (#8ceee2)\n",
        "U 2 (#caa173)\n",
        "L 1 (#1b58a2)\n",
        "U 2 (#caa171)\n",
        "R 2 (#7807d2)\n",
        "U 3 (#a77fa3)\n",
        "L 2 (#015232)\n",
        "U 2 (#7a21e3)\n",
    );

    #[test]
    fn test_part_a() {
        assert_eq!(
            trench_area(&parse_instructions(EXAMPLE_INPUT).unwrap().0),
            62
        );
    }

    #[test]
    fn test_part_b() {
        assert_eq!(
            trench_area(&parse_instructions(EXAMPLE_INPUT).unwrap().1),
            952_408_144_115
        );
    }
}
