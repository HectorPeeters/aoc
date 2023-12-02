use aoc::{error::Result, Solution};

use aoc::year2020::*;
use aoc::year2021::*;
use aoc::year2022::*;
use aoc::year2023::*;

fn run_solution(year: u32, day: u8) -> Result<()> {
    match (year, day) {
        (2020, 1) => Solution2020Day1::run(),
        (2020, 2) => Solution2020Day2::run(),
        (2020, 3) => Solution2020Day3::run(),
        (2020, 4) => Solution2020Day4::run(),
        (2020, 5) => Solution2020Day5::run(),
        (2020, 6) => Solution2020Day6::run(),
        (2021, 1) => Solution2021Day1::run(),
        (2021, 2) => Solution2021Day2::run(),
        (2021, 3) => Solution2021Day3::run(),
        (2021, 4) => Solution2021Day4::run(),
        (2021, 5) => Solution2021Day5::run(),
        (2021, 6) => Solution2021Day6::run(),
        (2021, 7) => Solution2021Day7::run(),
        (2021, 8) => Solution2021Day8::run(),
        (2022, 1) => Solution2022Day1::run(),
        (2023, 1) => Solution2023Day1::run(),
        (2023, 2) => Solution2023Day2::run(),
        _ => panic!("Unknown year/day combination"),
    }
}

fn main() -> Result<()> {
    run_solution(2023, 1)?;
    Ok(())
}
