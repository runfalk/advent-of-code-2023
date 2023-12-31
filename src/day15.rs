use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::Path;

fn hash(s: &str) -> u8 {
    s.bytes().fold(0, |h, b| h.wrapping_add(b).wrapping_mul(17))
}

fn part_a<'a>(lenses: impl Iterator<Item = &'a str>) -> usize {
    lenses.map(hash).map(usize::from).sum()
}

fn part_b<'a>(lenses: impl Iterator<Item = &'a str>) -> Result<usize> {
    let mut boxes = HashMap::<u8, Vec<(&str, usize)>>::new();
    for lens_str in lenses {
        if let Some(label) = lens_str.strip_suffix('-') {
            let lens_box = boxes.entry(hash(label)).or_default();
            let Some(i) = lens_box.iter().position(|&(l, _)| l == label) else {
                continue;
            };
            lens_box.remove(i);
        } else {
            let (label, focal_len_str) = lens_str
                .split_once('=')
                .ok_or_else(|| anyhow!("Invalid lens {:?}", lens_str))?;
            let focal_len = focal_len_str.parse()?;

            let lens_box = boxes.entry(hash(label)).or_default();
            let Some(i) = lens_box.iter().position(|&(l, _)| l == label) else {
                lens_box.push((label, focal_len));
                continue;
            };
            lens_box[i] = (label, focal_len);
        }
    }

    let mut focusing_power = 0;
    for (box_number, lens_box) in boxes.into_iter() {
        for (i, (_, focal_len)) in lens_box.into_iter().enumerate() {
            focusing_power += (usize::from(box_number) + 1) * (i + 1) * focal_len;
        }
    }
    Ok(focusing_power)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input = std::fs::read_to_string(path)?;
    let lenses = input.trim().split(',');
    Ok((part_a(lenses.clone()), part_b(lenses)?.into()))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(15, 516_070, 244_981);

    const EXAMPLE_INPUT: &'static str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(EXAMPLE_INPUT.split(',')), 1320);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(EXAMPLE_INPUT.split(',')).unwrap(), 145);
    }
}
