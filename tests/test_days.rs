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

// Needs to be ignored because my solution is slow :(
#[test]
#[ignore]
fn test_day5() -> Result<()> {
    assert_eq!(
        run_day(5, advent_of_code_2023::day5::main)?,
        (111_627_841, Some(69_323_688)),
    );
    Ok(())
}

#[test]
fn test_day6() -> Result<()> {
    assert_eq!(
        run_day(6, advent_of_code_2023::day6::main)?,
        (1_710_720, Some(35_349_468)),
    );
    Ok(())
}

#[test]
fn test_day7() -> Result<()> {
    assert_eq!(
        run_day(7, advent_of_code_2023::day7::main)?,
        (250_946_742, Some(251_824_095)),
    );
    Ok(())
}

#[test]
fn test_day8() -> Result<()> {
    assert_eq!(
        run_day(8, advent_of_code_2023::day8::main)?,
        (13_771, Some(13_129_439_557_681)),
    );
    Ok(())
}

#[test]
fn test_day9() -> Result<()> {
    assert_eq!(
        run_day(9, advent_of_code_2023::day9::main)?,
        (1_731_106_378, Some(1087)),
    );
    Ok(())
}

#[test]
fn test_day10() -> Result<()> {
    assert_eq!(
        run_day(10, advent_of_code_2023::day10::main)?,
        (6757, Some(523)),
    );
    Ok(())
}

#[test]
fn test_day11() -> Result<()> {
    assert_eq!(
        run_day(11, advent_of_code_2023::day11::main)?,
        (9_686_930, Some(630_728_425_490)),
    );
    Ok(())
}

#[test]
fn test_day12() -> Result<()> {
    assert_eq!(
        run_day(12, advent_of_code_2023::day12::main)?,
        (7670, Some(157_383_940_585_037)),
    );
    Ok(())
}
