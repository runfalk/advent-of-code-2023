use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug, PartialEq, Eq)]
struct Round {
    r: usize,
    g: usize,
    b: usize,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (id_str, rounds_str) = s
            .split_once(": ")
            .ok_or_else(|| anyhow!("Unable to find game ID"))?;

        if !id_str.starts_with("Game ") {
            return Err(anyhow!("Game doesn't start with 'Game '"));
        }

        let id: usize = id_str[5..]
            .parse()
            .map_err(|_| anyhow!("Unable to parse game ID {:?}", &id_str[5..]))?;
        let rounds = rounds_str
            .split("; ")
            .map(Round::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { id, rounds })
    }
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut red: usize = 0;
        let mut green: usize = 0;
        let mut blue: usize = 0;

        for cube_spec in s.split(", ") {
            let (num_str, color) = cube_spec
                .split_once(' ')
                .ok_or_else(|| anyhow!("Unable to parse color"))?;
            match color {
                "red" => {
                    red = num_str
                        .parse()
                        .map_err(|_| anyhow!("Unable to parse number of red"))?
                }
                "green" => {
                    green = num_str
                        .parse()
                        .map_err(|_| anyhow!("Unable to parse number of red"))?
                }
                "blue" => {
                    blue = num_str
                        .parse()
                        .map_err(|_| anyhow!("Unable to parse number of red"))?
                }
                _ => return Err(anyhow!("Invalid color {:?} in round", color)),
            }
        }

        Ok(Round {
            r: red,
            g: green,
            b: blue,
        })
    }
}

fn part_a(games: &[Game]) -> usize {
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    games
        .iter()
        .filter(|game| {
            game.rounds
                .iter()
                .all(|r| r.r <= red_limit && r.g <= green_limit && r.b <= blue_limit)
        })
        .map(|g| g.id)
        .sum()
}

fn part_b(games: &[Game]) -> usize {
    let mut cube_power_sum = 0;
    for game in games {
        let mut fewest_red = 0;
        let mut fewest_green = 0;
        let mut fewest_blue = 0;

        for round in game.rounds.iter() {
            fewest_red = fewest_red.max(round.r);
            fewest_green = fewest_green.max(round.g);
            fewest_blue = fewest_blue.max(round.b);
        }
        cube_power_sum += fewest_red * fewest_green * fewest_blue
    }
    cube_power_sum
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let games = BufReader::new(file)
        .lines()
        .map(|l| l?.parse::<Game>())
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    Ok((part_a(&games), part_b(&games).into()))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(2, 2776, 68638);

    const EXAMPLE_INPUT: &'static [&'static str] = &[
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];

    fn example_input() -> Vec<Game> {
        EXAMPLE_INPUT
            .iter()
            .map(|l| l.parse::<Game>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    #[test]
    fn test_parsing() {
        assert_eq!(
            EXAMPLE_INPUT[0].parse::<Game>().unwrap(),
            Game {
                id: 1,
                rounds: vec![
                    Round { r: 4, g: 0, b: 3 },
                    Round { r: 1, g: 2, b: 6 },
                    Round { r: 0, g: 2, b: 0 },
                ]
            }
        );
        assert_eq!(
            EXAMPLE_INPUT[1].parse::<Game>().unwrap(),
            Game {
                id: 2,
                rounds: vec![
                    Round { r: 0, g: 2, b: 1 },
                    Round { r: 1, g: 3, b: 4 },
                    Round { r: 0, g: 1, b: 1 },
                ]
            }
        );
        assert_eq!(
            EXAMPLE_INPUT[2].parse::<Game>().unwrap(),
            Game {
                id: 3,
                rounds: vec![
                    Round { r: 20, g: 8, b: 6 },
                    Round { r: 4, g: 13, b: 5 },
                    Round { r: 1, g: 5, b: 0 },
                ]
            }
        );
        assert_eq!(
            EXAMPLE_INPUT[3].parse::<Game>().unwrap(),
            Game {
                id: 4,
                rounds: vec![
                    Round { r: 3, g: 1, b: 6 },
                    Round { r: 6, g: 3, b: 0 },
                    Round { r: 14, g: 3, b: 15 },
                ]
            }
        );
        assert_eq!(
            EXAMPLE_INPUT[4].parse::<Game>().unwrap(),
            Game {
                id: 5,
                rounds: vec![Round { r: 6, g: 3, b: 1 }, Round { r: 1, g: 2, b: 2 }]
            }
        );
    }

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&example_input()), 8);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&example_input()), 2286);
    }
}
