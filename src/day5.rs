use anyhow::{anyhow, Result};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: RangeSet,
    soil_to_fertilizer: RangeSet,
    fertilizer_to_water: RangeSet,
    water_to_light: RangeSet,
    light_to_temperature: RangeSet,
    temperature_to_humidity: RangeSet,
    humidity_to_location: RangeSet,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    src: usize,
    len: usize,
    dst: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RangeSet {
    ranges: Vec<Range>,
}

impl RangeSet {
    fn new(ranges: &[Range]) -> Self {
        let mut ranges = ranges.to_vec();
        ranges.sort();
        Self { ranges }
    }

    fn map(&self, src: usize) -> usize {
        for r in self.ranges.iter() {
            if (r.src..r.src + r.len).contains(&src) {
                return r.dst + src - r.src;
            }
        }
        src
    }
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut seeds = Vec::new();
        let mut seed_to_soil: Vec<Range> = Vec::new();
        let mut soil_to_fertilizer: Vec<Range> = Vec::new();
        let mut fertilizer_to_water: Vec<Range> = Vec::new();
        let mut water_to_light: Vec<Range> = Vec::new();
        let mut light_to_temperature: Vec<Range> = Vec::new();
        let mut temperature_to_humidity: Vec<Range> = Vec::new();
        let mut humidity_to_location: Vec<Range> = Vec::new();
        for group in s.split("\n\n") {
            let Some((label, content)) = group.split_once(':') else {
                return Err(anyhow!("Unable to determine group label in alamac"));
            };

            match label {
                "seeds" => {
                    seeds = content
                        .split_whitespace()
                        .map(|n| n.parse::<usize>())
                        .collect::<Result<_, _>>()?;
                }
                "seed-to-soil map" => {
                    for line in content.trim_start().lines() {
                        seed_to_soil.push(line.parse()?);
                    }
                }
                "soil-to-fertilizer map" => {
                    for line in content.trim_start().lines() {
                        soil_to_fertilizer.push(line.parse()?);
                    }
                }
                "fertilizer-to-water map" => {
                    for line in content.trim_start().lines() {
                        fertilizer_to_water.push(line.parse()?);
                    }
                }
                "water-to-light map" => {
                    for line in content.trim_start().lines() {
                        water_to_light.push(line.parse()?);
                    }
                }
                "light-to-temperature map" => {
                    for line in content.trim_start().lines() {
                        light_to_temperature.push(line.parse()?);
                    }
                }
                "temperature-to-humidity map" => {
                    for line in content.trim_start().lines() {
                        temperature_to_humidity.push(line.parse()?);
                    }
                }
                "humidity-to-location map" => {
                    for line in content.trim_start().lines() {
                        humidity_to_location.push(line.parse()?);
                    }
                }
                _ => {
                    return Err(anyhow!("Unknown group label {:?}", label));
                }
            }
        }

        Ok(Self {
            seeds,
            seed_to_soil: RangeSet::new(&seed_to_soil),
            soil_to_fertilizer: RangeSet::new(&soil_to_fertilizer),
            fertilizer_to_water: RangeSet::new(&fertilizer_to_water),
            water_to_light: RangeSet::new(&water_to_light),
            light_to_temperature: RangeSet::new(&light_to_temperature),
            temperature_to_humidity: RangeSet::new(&temperature_to_humidity),
            humidity_to_location: RangeSet::new(&humidity_to_location),
        })
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let nums = s.split_whitespace().collect::<Vec<_>>();
        if nums.len() != 3 {
            return Err(anyhow!("HM {:?}", nums));
        }

        let dst = nums[0].parse()?;
        let src = nums[1].parse()?;
        let len = nums[2].parse()?;

        Ok(Range { src, dst, len })
    }
}

fn part_a(almanac: &Almanac) -> usize {
    let mut lowest_location = usize::MAX;
    for seed in almanac.seeds.iter().copied() {
        let soil = almanac.seed_to_soil.map(seed);
        let fertilizer = almanac.soil_to_fertilizer.map(soil);
        let water = almanac.fertilizer_to_water.map(fertilizer);
        let light = almanac.water_to_light.map(water);
        let temperature = almanac.light_to_temperature.map(light);
        let humidity = almanac.temperature_to_humidity.map(temperature);
        let location = almanac.humidity_to_location.map(humidity);
        lowest_location = lowest_location.min(location);
    }
    lowest_location
}

fn part_b(almanac: &Almanac) -> usize {
    let mut lowest_location = usize::MAX;
    for s in almanac.seeds.chunks(2) {
        for seed in s[0]..s[0] + s[1] {
            let soil = almanac.seed_to_soil.map(seed);
            let fertilizer = almanac.soil_to_fertilizer.map(soil);
            let water = almanac.fertilizer_to_water.map(fertilizer);
            let light = almanac.water_to_light.map(water);
            let temperature = almanac.light_to_temperature.map(light);
            let humidity = almanac.temperature_to_humidity.map(temperature);
            let location = almanac.humidity_to_location.map(humidity);
            lowest_location = lowest_location.min(location);
        }
    }
    lowest_location
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = std::fs::read_to_string(path)?;
    let almanac = file.parse()?;
    Ok((part_a(&almanac), part_b(&almanac).into()))
}

#[cfg(test)]
mod test {
    use super::*;

    // Needs to be ignored because my solution is slow :(
    test_real_input!(
        #[ignore]
        5,
        111_627_841,
        69_323_688
    );

    const EXAMPLE_INPUT: &'static str = concat!(
        "seeds: 79 14 55 13\n",
        "\n",
        "seed-to-soil map:\n",
        "50 98 2\n",
        "52 50 48\n",
        "\n",
        "soil-to-fertilizer map:\n",
        "0 15 37\n",
        "37 52 2\n",
        "39 0 15\n",
        "\n",
        "fertilizer-to-water map:\n",
        "49 53 8\n",
        "0 11 42\n",
        "42 0 7\n",
        "57 7 4\n",
        "\n",
        "water-to-light map:\n",
        "88 18 7\n",
        "18 25 70\n",
        "\n",
        "light-to-temperature map:\n",
        "45 77 23\n",
        "81 45 19\n",
        "68 64 13\n",
        "\n",
        "temperature-to-humidity map:\n",
        "0 69 1\n",
        "1 0 69\n",
        "\n",
        "humidity-to-location map:\n",
        "60 56 37\n",
        "56 93 4\n",
    );

    fn example_input() -> Almanac {
        EXAMPLE_INPUT.parse().unwrap()
    }

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&example_input()), 35);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&example_input()), 46);
    }
}
