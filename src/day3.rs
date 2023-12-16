use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct Schematic {
    /// Ordered list of numbers as they appear in the schematic
    nums: Vec<usize>,

    /// Map from position to index in the list of numbers
    pos_to_num_ids: HashMap<(usize, usize), usize>,

    /// Map from coordinate to symbol value for all symbols within the schematic
    symbols: HashMap<(usize, usize), char>,
}

impl Schematic {
    fn parse(schematic: &[Vec<char>]) -> Self {
        // Find all symbols in the first pass
        let symbols = schematic
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().filter_map(move |(x, c)| {
                    (*c != '.' && c.is_ascii_punctuation()).then_some(((x, y), *c))
                })
            })
            .collect();

        // Find all numbers and their positions in the second pass
        let mut nums = Vec::new();
        let mut pos_to_num_ids = HashMap::new();
        for (y, line) in schematic.iter().enumerate() {
            let mut line_iter = line.iter().enumerate().peekable();
            loop {
                // If we find a digit, try to extract all the entire number
                let num_id = nums.len();
                let mut num = 0;
                while let Some((x, d)) = line_iter.next().filter(|(_, d)| d.is_ascii_digit()) {
                    pos_to_num_ids.insert((x, y), num_id);
                    num = 10 * num + d.to_digit(10).unwrap() as usize;
                }

                // If we found a number, add it to our list of numbers
                if num > 0 {
                    nums.push(num);
                }

                // Check if we have reached the end of the line
                if line_iter.peek().is_none() {
                    break;
                }
            }
        }

        Self {
            nums,
            pos_to_num_ids,
            symbols,
        }
    }

    /// Return all numbers that are adjacent to the given positions
    fn adjacent_numbers<T: IntoIterator<Item = (usize, usize)>>(&self, positions: T) -> Vec<usize> {
        let mut nums = Vec::new();
        let mut used_nums = HashSet::new();
        for pos in positions.into_iter().flat_map(iter_adjacent) {
            let Some(number_id) = self.pos_to_num_ids.get(&pos).copied() else {
                continue;
            };
            if used_nums.insert(number_id) {
                nums.push(self.nums[number_id])
            }
        }
        nums
    }
}

/// Return all adjacent cells to the given position
fn iter_adjacent((x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    (y.saturating_sub(1)..=(y + 1)).flat_map(move |ny| {
        (x.saturating_sub(1)..=(x + 1))
            .filter_map(move |nx| (x != nx || y != ny).then_some((nx, ny)))
    })
}

fn part_a(schematic: &Schematic) -> usize {
    schematic
        .adjacent_numbers(schematic.symbols.keys().copied())
        .into_iter()
        .sum()
}

fn part_b(schematic: &Schematic) -> usize {
    let mut sum = 0;
    for (pos, symbol) in schematic.symbols.iter() {
        if *symbol != '*' {
            continue;
        }
        let adjacent_numbers: Vec<usize> = schematic.adjacent_numbers([*pos]);
        if adjacent_numbers.len() == 2 {
            sum += adjacent_numbers[0] * adjacent_numbers[1];
        }
    }
    sum
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let raw_schematic = BufReader::new(file)
        .lines()
        .map(|l| Ok(l?.chars().collect()))
        .collect::<Result<Vec<Vec<char>>, anyhow::Error>>()?;
    let schematic = Schematic::parse(&raw_schematic);

    Ok((part_a(&schematic), Some(part_b(&schematic))))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(3, 557_705, 84_266_818);

    fn example_schematic() -> Schematic {
        let raw_schematic = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .into_iter()
        .map(|l| l.chars().collect())
        .collect::<Vec<_>>();
        Schematic::parse(&raw_schematic)
    }

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&example_schematic()), 4361);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&example_schematic()), 467835);
    }
}
