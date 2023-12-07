use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Card(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand([Card; 5]);

impl Card {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Self(14)),
            'K' => Some(Self(13)),
            'Q' => Some(Self(12)),
            'J' => Some(Self(11)),
            'T' => Some(Self(10)),
            '2'..='9' => Some(Self(c.to_digit(10).unwrap() as usize)),
            _ => None,
        }
    }

    fn jack_into_joker(self) -> Self {
        if self.0 == 11 {
            Self(1)
        } else {
            self
        }
    }
}

impl Hand {
    fn count_cards(&self) -> HashMap<Card, usize> {
        let mut counts = HashMap::new();
        for card in self.0.iter().copied() {
            *counts.entry(card).or_default() += 1;
        }
        counts
    }

    fn jacks_into_jokers(&self) -> Self {
        Self([
            self.0[0].jack_into_joker(),
            self.0[1].jack_into_joker(),
            self.0[2].jack_into_joker(),
            self.0[3].jack_into_joker(),
            self.0[4].jack_into_joker(),
        ])
    }

    fn tier(&self) -> usize {
        let mut card_counts = self.count_cards();
        let num_jokers = card_counts.remove(&Card(1)).unwrap_or(0);

        // Sort the card counts in reverse order
        let mut sorted_card_counts = card_counts.values().copied().collect::<Vec<_>>();
        sorted_card_counts.sort();
        sorted_card_counts.reverse();

        // We only need to look at the first two values to determine the hand. If we have jokers we
        // we can simply add them to the most common card since that will always net us the best
        // hand
        let a = sorted_card_counts.get(0).copied().unwrap_or(0) + num_jokers;
        let b = sorted_card_counts.get(1).copied().unwrap_or(0);

        match (a, b) {
            (5, 0) => 6, // Five of a kind
            (4, 1) => 5, // Four of a kind
            (3, 2) => 4, // Full house
            (3, 1) => 3, // Three of a kind
            (2, 2) => 2, // Two pairs
            (2, 1) => 1, // One pair
            (1, 1) => 0, // All unique
            _ => panic!("We should never get here"),
        }
    }
}

fn parse_hand_with_bid(s: &str) -> Result<(Hand, usize)> {
    let Some((hand_str, bid_str)) = s.split_once(' ') else {
        return Err(anyhow!("Unable to find bid in {:?}", s));
    };
    let cards = hand_str
        .chars()
        .filter_map(Card::from_char)
        .collect::<Vec<_>>();
    Ok((
        Hand(cards.try_into().map_err(|c: Vec<Card>| {
            anyhow!("Invalid number of cards, expected 5 got {}", c.len())
        })?),
        bid_str.parse()?,
    ))
}

fn total_winnings(hands: &[(Hand, usize)], jacks_into_jokers: bool) -> usize {
    let mut sorted_hands = if jacks_into_jokers {
        hands
            .iter()
            .cloned()
            .map(|(hand, bid)| (hand.jacks_into_jokers(), bid))
            .collect::<Vec<_>>()
    } else {
        hands.to_vec()
    };
    sorted_hands.sort_by_key(|(hand, _)| (hand.tier(), hand.0));

    let mut winnings = 0;
    for (i, (_, bid)) in sorted_hands.into_iter().enumerate() {
        winnings += (i + 1) * bid;
    }
    winnings
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let hands = BufReader::new(file)
        .lines()
        .map(|l| parse_hand_with_bid(&l?))
        .collect::<Result<Vec<(Hand, usize)>, _>>()?;
    Ok((
        total_winnings(&hands, false),
        total_winnings(&hands, true).into(),
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hand_tier() {
        assert_eq!(parse_hand_with_bid("2468T 0").unwrap().0.tier(), 0);
        assert_eq!(parse_hand_with_bid("24682 0").unwrap().0.tier(), 1);
        assert_eq!(parse_hand_with_bid("24642 0").unwrap().0.tier(), 2);
        assert_eq!(parse_hand_with_bid("22682 0").unwrap().0.tier(), 3);
        assert_eq!(parse_hand_with_bid("22662 0").unwrap().0.tier(), 4);
        assert_eq!(parse_hand_with_bid("22622 0").unwrap().0.tier(), 5);
        assert_eq!(parse_hand_with_bid("22222 0").unwrap().0.tier(), 6);
    }

    fn example_input() -> Vec<(Hand, usize)> {
        vec![
            parse_hand_with_bid("32T3K 765").unwrap(),
            parse_hand_with_bid("T55J5 684").unwrap(),
            parse_hand_with_bid("KK677 28").unwrap(),
            parse_hand_with_bid("KTJJT 220").unwrap(),
            parse_hand_with_bid("QQQJA 483").unwrap(),
        ]
    }

    #[test]
    fn test_part_a() {
        assert_eq!(total_winnings(&example_input(), false), 6440);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(total_winnings(&example_input(), true), 5905);
    }
}
