use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::path::Path;

fn parse_galaxies(s: &str) -> Result<HashSet<(usize, usize)>> {
    let mut map = HashSet::new();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    map.insert((x, y));
                }
                '.' => {}
                _ => {
                    return Err(anyhow!(
                        "Invalid map character {:?} at line {} position {}",
                        c,
                        y,
                        x
                    ));
                }
            }
        }
    }

    // Add extra columns based on empty space
    Ok(map)
}

fn expand_void(
    galaxies: &HashSet<(usize, usize)>,
    void_expansion_factor: usize,
) -> Result<HashSet<(usize, usize)>> {
    if void_expansion_factor == 0 {
        return Err(anyhow!("Void expansion factor must be greater than 0"));
    }

    let max_x = galaxies.iter().copied().map(|(x, _)| x).max().unwrap_or(0);
    let max_y = galaxies.iter().copied().map(|(_, y)| y).max().unwrap_or(0);

    let void_columns = (0..=max_x)
        .filter(|&x| (0..=max_y).all(move |y| !galaxies.contains(&(x, y))))
        .collect::<Vec<_>>();
    let void_rows = (0..=max_y)
        .filter(|&y| (0..=max_x).all(move |x| !galaxies.contains(&(x, y))))
        .collect::<Vec<_>>();

    let mut expanded_galaxies = HashSet::new();
    for (x, y) in galaxies.iter().copied() {
        let num_void_columns = void_columns
            .iter()
            .take_while(|x_limit| &x > x_limit)
            .count();
        let num_void_rows = void_rows.iter().take_while(|y_limit| &y > y_limit).count();
        expanded_galaxies.insert((
            x + num_void_columns * (void_expansion_factor - 1),
            y + num_void_rows * (void_expansion_factor - 1),
        ));
    }

    Ok(expanded_galaxies)
}

fn sum_pairwise_distances(
    galaxies: &HashSet<(usize, usize)>,
    void_expansion_factor: usize,
) -> Result<usize> {
    let mut expanded_galaxies = expand_void(galaxies, void_expansion_factor)?
        .into_iter()
        .collect::<Vec<_>>();

    let mut sum = 0;
    while let Some(a) = expanded_galaxies.pop() {
        for b in expanded_galaxies.iter().copied() {
            sum += a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
        }
    }
    Ok(sum)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let map_str = std::fs::read_to_string(path)?;
    let galaxies = parse_galaxies(&map_str)?;
    Ok((
        sum_pairwise_distances(&galaxies, 2)?,
        sum_pairwise_distances(&galaxies, 1_000_000)?.into(),
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(11, 9_686_930, 630_728_425_490);

    const EXAMPLE_A: &'static str = concat!(
        "...#......\n",
        ".......#..\n",
        "#.........\n",
        "..........\n",
        "......#...\n",
        ".#........\n",
        ".........#\n",
        "..........\n",
        ".......#..\n",
        "#...#.....\n",
    );

    const EXAMPLE_A_EXPANDED: &'static str = concat!(
        "....#........\n",
        ".........#...\n",
        "#............\n",
        ".............\n",
        ".............\n",
        "........#....\n",
        ".#...........\n",
        "............#\n",
        ".............\n",
        ".............\n",
        ".........#...\n",
        "#....#.......\n",
    );

    #[test]
    fn test_expand_void() {
        assert_eq!(
            expand_void(&parse_galaxies(EXAMPLE_A).unwrap(), 2).unwrap(),
            parse_galaxies(EXAMPLE_A_EXPANDED).unwrap(),
        );
    }

    #[test]
    fn test_sum_distances() {
        let galaxies = parse_galaxies(EXAMPLE_A).unwrap();
        assert_eq!(sum_pairwise_distances(&galaxies, 2).unwrap(), 374);
        assert_eq!(sum_pairwise_distances(&galaxies, 10).unwrap(), 1030);
        assert_eq!(sum_pairwise_distances(&galaxies, 100).unwrap(), 8410);
    }
}
