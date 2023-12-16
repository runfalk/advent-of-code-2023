use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn find_all_digits(s: &str, include_named: bool) -> impl Iterator<Item = usize> + '_ {
    (0..s.len()).filter_map(move |i| {
        if s[i..].starts_with('1') || (include_named && s[i..].starts_with("one")) {
            Some(1)
        } else if s[i..].starts_with('2') || (include_named && s[i..].starts_with("two")) {
            Some(2)
        } else if s[i..].starts_with('3') || (include_named && s[i..].starts_with("three")) {
            Some(3)
        } else if s[i..].starts_with('4') || (include_named && s[i..].starts_with("four")) {
            Some(4)
        } else if s[i..].starts_with('5') || (include_named && s[i..].starts_with("five")) {
            Some(5)
        } else if s[i..].starts_with('6') || (include_named && s[i..].starts_with("six")) {
            Some(6)
        } else if s[i..].starts_with('7') || (include_named && s[i..].starts_with("seven")) {
            Some(7)
        } else if s[i..].starts_with('8') || (include_named && s[i..].starts_with("eight")) {
            Some(8)
        } else if s[i..].starts_with('9') || (include_named && s[i..].starts_with("nine")) {
            Some(9)
        } else {
            None
        }
    })
}

fn find_calibration_value(line: &str, include_named: bool) -> Result<usize> {
    let first = find_all_digits(line, include_named)
        .next()
        .ok_or_else(|| anyhow!("Unable to parse first digit {:?}", line))?;
    let second = find_all_digits(line, include_named)
        .last()
        .ok_or_else(|| anyhow!("Unable to parse second digit {:?}", line))?;
    Ok(10 * first + second)
}

fn calibration_value_sum(lines: &[String], include_named: bool) -> Result<usize> {
    lines
        .iter()
        .map(|line| find_calibration_value(line, include_named))
        .sum()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let lines = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    Ok((
        calibration_value_sum(&lines, false)?,
        Some(calibration_value_sum(&lines, true)?),
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(1, 55090, 54845);

    #[test]
    fn test_edge_cases() {
        assert_eq!(find_calibration_value("eighthree", true).unwrap(), 83);
        assert_eq!(find_calibration_value("sevenine", true).unwrap(), 79);
    }

    #[test]
    fn test_part_a() {
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .into_iter()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        assert_eq!(calibration_value_sum(&input, false).unwrap(), 142);
    }

    #[test]
    fn test_part_b() {
        let input = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .into_iter()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
        assert_eq!(calibration_value_sum(&input, true).unwrap(), 281);
    }
}
