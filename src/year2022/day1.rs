use itertools::Itertools;

use crate::{aoc_test, error::Result, Solution};

pub struct Solution2022Day1;
aoc_test!(Solution2022Day1, 74198, 209_914);

impl Solution for Solution2022Day1 {
    const YEAR: u32 = 2022;
    const DAY: u8 = 1;

    type Data = Vec<Vec<u32>>;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        Ok(input
            .split("\n\n")
            .map(|x| x.split('\n').filter_map(|y| y.parse().ok()).collect())
            .collect())
    }

    fn part1(data: &Self::Data) -> Result<Self::Output> {
        Ok(data.iter().map(|x| x.iter().sum()).max().unwrap())
    }

    fn part2(data: &Self::Data) -> Result<Self::Output> {
        Ok(data
            .iter()
            .map(|x| x.iter().sum::<u32>())
            .sorted_unstable()
            .rev()
            .take(3)
            .sum())
    }
}
