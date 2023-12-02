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
