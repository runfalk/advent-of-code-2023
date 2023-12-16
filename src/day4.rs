use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
struct Card {
    nums: Vec<usize>,
    winning_nums: Vec<usize>,
}

impl Card {
    fn num_matches(&self) -> usize {
        self.nums
            .iter()
            .filter(|n| self.winning_nums.contains(n))
            .count()
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // We ignore the Card X: bit since it's not used for anything
        let Some((_, card_str)) = s.split_once(": ") else {
            return Err(anyhow!("Unable to find : separator"));
        };

        let Some((nums_str, winning_nums_str)) = card_str.split_once(" | ") else {
            return Err(anyhow!("Unable to find | separator"));
        };

        let nums = nums_str
            .split_whitespace()
            .map(|num_str| {
                num_str
                    .parse()
                    .map_err(|_| anyhow!("Unable to parse number"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let winning_nums = winning_nums_str
            .split_whitespace()
            .map(|num_str| {
                num_str
                    .parse()
                    .map_err(|_| anyhow!("Unable to parse winning number"))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { nums, winning_nums })
    }
}

fn part_a(cards: &[Card]) -> usize {
    cards
        .iter()
        .map(|c| {
            let num_matches = c.num_matches() as u32;
            if num_matches == 0 {
                0
            } else {
                2usize.pow(num_matches - 1)
            }
        })
        .sum()
}

fn part_b(cards: &[Card]) -> usize {
    let mut card_multiplier = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let num_matches = card.num_matches();
        for j in (i + 1)..(i + 1 + num_matches).min(cards.len()) {
            card_multiplier[j] += card_multiplier[i];
        }
    }
    card_multiplier.into_iter().sum()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let cards = BufReader::new(file)
        .lines()
        .map(|l| l?.parse())
        .collect::<Result<Vec<Card>, _>>()?;
    Ok((part_a(&cards), part_b(&cards).into()))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(4, 28_750, 10_212_704);

    fn example_input() -> Vec<Card> {
        [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .into_iter()
        .map(|l| l.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
    }

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&example_input()), 13);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&example_input()), 30);
    }
}
