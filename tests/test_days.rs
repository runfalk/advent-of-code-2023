use anyhow::Result;
use std::path::Path;

fn run_day<A, B>(day: usize, f: fn(&Path) -> Result<(A, Option<B>)>) -> Result<(A, Option<B>)> {
    f(format!("data/day{}.txt", day).as_ref())
}

#[test]
fn test_day1() -> Result<()> {
    assert_eq!(
        run_day(1, advent_of_code_2023::day1::main)?,
        (55090, Some(54845)),
    );
    Ok(())
}

#[test]
fn test_day2() -> Result<()> {
    assert_eq!(
        run_day(2, advent_of_code_2023::day2::main)?,
        (2776, Some(68638)),
    );
    Ok(())
}

#[test]
fn test_day3() -> Result<()> {
    assert_eq!(
        run_day(3, advent_of_code_2023::day3::main)?,
        (557_705, Some(84_266_818)),
    );
    Ok(())
}

#[test]
fn test_day4() -> Result<()> {
    assert_eq!(
        run_day(4, advent_of_code_2023::day4::main)?,
        (28_750, Some(10_212_704)),
    );
    Ok(())
}
