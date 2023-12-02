#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]

pub mod error;
pub mod year2020;
pub mod year2021;
pub mod year2022;
pub mod year2023;

use error::Result;

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

        let data = Self::parse(&input)?;

        println!("Running solution for {} day {}", Self::YEAR, Self::DAY);
        println!("Part 1: {}", Self::part1(&data)?);
        println!("Part 2: {}", Self::part2(&data)?);

        Ok(())
    }
}

#[macro_export]
macro_rules! aoc_test {
    ($solution:ident, $part1:expr, $part2:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            fn load_input() -> Result<String> {
                Ok(std::fs::read_to_string(format!(
                    "input/{}/day{}.txt",
                    $solution::YEAR,
                    $solution::DAY
                ))?)
            }

            #[test]
            fn part1() -> Result<()> {
                let input = load_input()?;
                let data = $solution::parse(&input)?;
                assert_eq!($solution::part1(&data)?, $part1);

                Ok(())
            }

            #[test]
            fn part2() -> Result<()> {
                let input = load_input()?;
                let data = $solution::parse(&input)?;
                assert_eq!($solution::part2(&data)?, $part2);

                Ok(())
            }
        }
    };
}
