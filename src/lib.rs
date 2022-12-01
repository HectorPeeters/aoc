#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]

pub mod error;
pub mod year2020;
pub mod year2021;
pub mod year2022;

use error::Result;

pub fn read_input_file(path: &str) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

pub trait Solution {
    fn parse(&mut self) -> Result<()>;

    fn part1(&self) -> Result<u64>;

    fn part2(&self) -> Result<u64>;
}

#[derive(Clone, Copy)]
pub struct SolutionId {
    pub year: usize,
    pub day: usize,
}

impl From<(usize, usize)> for SolutionId {
    fn from(value: (usize, usize)) -> Self {
        Self {
            year: value.0,
            day: value.1,
        }
    }
}

#[macro_export]
macro_rules! aoc_test {
    ($solution:ident, $part1:expr, $part2:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn part1() -> Result<()> {
                let mut solution = $solution::default();
                solution.parse()?;
                assert_eq!(solution.part1()?, $part1);

                Ok(())
            }

            #[test]
            fn part2() -> Result<()> {
                let mut solution = $solution::default();
                solution.parse()?;
                assert_eq!(solution.part2()?, $part2);

                Ok(())
            }
        }
    };
}
