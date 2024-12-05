#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

pub mod error;
pub mod year2020;
pub mod year2021;
pub mod year2022;
pub mod year2023;
pub mod year2024;

use error::Result;

fn measure<T>(f: impl Fn() -> Result<T>) -> Result<(T, std::time::Duration)> {
    const N: u32 = 10;

    let result = f()?;

    let mut duration = std::time::Duration::new(0, 0);
    for _ in 0..N {
        let start = std::time::Instant::now();
        f()?;
        duration += start.elapsed();
    }

    Ok((result, duration / N))
}

pub trait Solution {
    const YEAR: u32;
    const DAY: u8;

    type Data;
    type Output: std::fmt::Display;

    fn parse(input: &str) -> Result<Self::Data>;

    fn part1(input: &Self::Data) -> Result<Self::Output>;

    fn part2(input: &Self::Data) -> Result<Self::Output>;

    fn run() -> Result<()> {
        let input = std::fs::read_to_string(format!("input/{}/day{}.txt", Self::YEAR, Self::DAY))?;

        let (data, duration) = measure(|| Self::parse(&input))?;

        println!("Running solution for {} day {}", Self::YEAR, Self::DAY);
        let (part1, part1_duration) = measure(|| Self::part1(&data))?;
        println!("Part 1: {part1}");
        let (part2, part2_duration) = measure(|| Self::part2(&data))?;
        println!("Part 2: {part2}");

        println!("\nBenchmark:");
        println!("Parse: {duration:?}");
        println!("Part 1: {part1_duration:?}");
        println!("Part 2: {part2_duration:?}");

        Ok(())
    }
}

#[macro_export]
macro_rules! aoc_test {
    ($solution:ident, $part1:expr, $part2:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            fn load_input() -> $crate::error::Result<String> {
                Ok(std::fs::read_to_string(format!(
                    "input/{}/day{}.txt",
                    $solution::YEAR,
                    $solution::DAY
                ))?)
            }

            #[test]
            fn part1() -> $crate::error::Result<()> {
                let input = load_input()?;
                let data = $solution::parse(&input)?;
                assert_eq!($solution::part1(&data)?, $part1);

                Ok(())
            }

            #[test]
            fn part2() -> $crate::error::Result<()> {
                let input = load_input()?;
                let data = $solution::parse(&input)?;
                assert_eq!($solution::part2(&data)?, $part2);

                Ok(())
            }
        }
    };
}
