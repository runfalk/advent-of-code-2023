use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn iter_pairs<T: Clone>(seq: &[T]) -> impl Iterator<Item = (T, T)> + '_ {
    seq.iter().cloned().zip(seq.iter().skip(1).cloned())
}

fn next_value(seq: &[isize]) -> Option<isize> {
    let mut stack: Vec<Vec<isize>> = Vec::new();
    stack.push(seq.to_vec());

    while iter_pairs(stack.last()?).any(|(a, b)| a != b) {
        stack.push(iter_pairs(stack.last()?).map(|(a, b)| b - a).collect());
    }

    while stack.len() > 1 {
        let diff = *stack.pop()?.last()?;
        let parent = stack.last_mut()?;
        parent.push(parent.last()? + diff);
    }
    Some(*stack[0].last()?)
}

fn part_a(seqs: &[Vec<isize>]) -> Result<isize> {
    let mut sum = 0;
    for seq in seqs {
        let Some(n) = next_value(seq) else {
            return Err(anyhow!(
                "Unable to determine next value in sequence {:?}",
                seq
            ));
        };
        sum += n;
    }
    Ok(sum)
}

fn part_b(seqs: &[Vec<isize>]) -> Result<isize> {
    let mut sum = 0;
    for seq in seqs {
        let mut reverse_seq = seq.to_vec();
        reverse_seq.reverse();
        let Some(n) = next_value(&reverse_seq) else {
            return Err(anyhow!(
                "Unable to determine next value in sequence {:?}",
                seq
            ));
        };
        sum += n;
    }
    Ok(sum)
}

fn parse_seq(s: &str) -> Result<Vec<isize>> {
    s.split_whitespace()
        .map(|num_str| Ok(num_str.parse()?))
        .collect::<Result<Vec<_>, _>>()
}

pub fn main(path: &Path) -> Result<(isize, Option<isize>)> {
    let file = File::open(path)?;
    let seqs = BufReader::new(file)
        .lines()
        .map(|l| parse_seq(&l?))
        .collect::<Result<Vec<Vec<isize>>, _>>()?;
    Ok((part_a(&seqs)?, part_b(&seqs)?.into()))
}

#[cfg(test)]
mod test {
    use super::*;

    const A: &'static [isize] = &[0, 3, 6, 9, 12, 15];
    const B: &'static [isize] = &[1, 3, 6, 10, 15, 21];
    const C: &'static [isize] = &[10, 13, 16, 21, 30, 45];

    #[test]
    fn test_next_digit() {
        assert_eq!(next_value(A), Some(18));
        assert_eq!(next_value(B), Some(28));
        assert_eq!(next_value(C), Some(68));
    }

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&[A.to_vec(), B.to_vec(), C.to_vec()]).unwrap(), 114);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&[A.to_vec(), B.to_vec(), C.to_vec()]).unwrap(), 2);
    }
}
