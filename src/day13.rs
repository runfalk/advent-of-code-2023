use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::path::Path;

#[derive(Clone, PartialEq, Eq)]
struct Note {
    width: usize,
    height: usize,
    rocks: HashSet<(usize, usize)>,
}

impl Note {
    fn toggle_rock(&self, x: usize, y: usize) -> Self {
        let mut fixed_note = self.clone();
        match fixed_note.rocks.take(&(x, y)) {
            Some(_) => {}
            None => {
                fixed_note.rocks.insert((x, y));
            }
        }
        fixed_note
    }

    fn is_rock(&self, x: usize, y: usize) -> bool {
        self.rocks.contains(&(x, y))
    }
}

fn parse_notes(s: &str) -> Result<Vec<Note>> {
    let mut notes = Vec::new();
    for note_str in s.split("\n\n") {
        let mut width = 0;
        let mut height = 0;
        let mut rocks = HashSet::new();
        for (y, line) in note_str.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        rocks.insert((x, y));
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
        notes.push(Note {
            width,
            height,
            rocks,
        });
    }
    Ok(notes)
}

fn find_mirror(note: &Note, ignore: Option<usize>) -> Option<usize> {
    // Find horizontal mirror
    for y in 0..(note.height - 1) {
        let num_lines_required = (y + 1).min(note.height - y - 1);
        let mut is_mirror = true;
        for dy in 0..num_lines_required {
            let y_above = y - dy;
            let y_below = y + 1 + dy;

            if (0..note.width).any(|x| note.is_rock(x, y_above) != note.is_rock(x, y_below)) {
                is_mirror = false;
                break;
            }
        }
        let mirror = Some(100 * (y + 1));
        if is_mirror && mirror != ignore {
            return mirror;
        }
    }

    // Find vertical mirror
    for x in 0..(note.width - 1) {
        let num_cols_required = (x + 1).min(note.width - x - 1);
        let mut is_mirror = true;
        for dx in 0..num_cols_required {
            let x_left = x - dx;
            let x_right = x + 1 + dx;

            if (0..note.height).any(|y| note.is_rock(x_left, y) != note.is_rock(x_right, y)) {
                is_mirror = false;
                break;
            }
        }
        let mirror = Some(x + 1);
        if is_mirror && mirror != ignore {
            return mirror;
        }
    }
    None
}

fn part_a(notes: &[Note]) -> Result<usize> {
    let mut sum = 0;
    for note in notes {
        sum += find_mirror(note, None).ok_or_else(|| anyhow!("No mirror found"))?;
    }
    Ok(sum)
}

fn part_b(notes: &[Note]) -> Result<usize> {
    let mut sum_without_smudges = 0;
    'outer: for note in notes {
        let mirror_with_smudge =
            find_mirror(note, None).ok_or_else(|| anyhow!("No mirror found"))?;

        // Fix all possible smudges
        for y in 0..note.height {
            for x in 0..note.width {
                // Fix possible smudge
                let fixed_note = note.toggle_rock(x, y);
                let Some(mirror_without_smudge) =
                    find_mirror(&fixed_note, Some(mirror_with_smudge))
                else {
                    continue;
                };

                sum_without_smudges += mirror_without_smudge;
                continue 'outer;
            }
        }
        return Err(anyhow!("Failed to find new mirror after fixing smudges"));
    }
    Ok(sum_without_smudges)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let notes_str = std::fs::read_to_string(path)?;
    let notes = parse_notes(&notes_str)?;
    Ok((part_a(&notes)?, part_b(&notes)?.into()))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(13, 41_859, 30_842);

    fn example_input() -> Vec<Note> {
        parse_notes(concat!(
            "#.##..##.\n",
            "..#.##.#.\n",
            "##......#\n",
            "##......#\n",
            "..#.##.#.\n",
            "..##..##.\n",
            "#.#.##.#.\n",
            "\n",
            "#...##..#\n",
            "#....#..#\n",
            "..##..###\n",
            "#####.##.\n",
            "#####.##.\n",
            "..##..###\n",
            "#....#..#\n",
        ))
        .unwrap()
    }

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&example_input()).unwrap(), 405);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&example_input()).unwrap(), 400);
    }
}
