use crate::error::Result;
use crate::{aoc_test, Solution};
use std::cmp::PartialOrd;

pub struct Solution2021Day1;
aoc_test!(Solution2021Day1, 1185, 1158);

fn count_incrementing<I, T>(data: I) -> u32
where
    T: PartialOrd,
    I: Iterator<Item = T> + Clone,
{
    data.clone()
        .zip(data.skip(1))
        .filter(|(first, second)| first < second)
        .count() as u32
}

impl Solution for Solution2021Day1 {
    const YEAR: u32 = 2021;
    const DAY: u8 = 1;

    type Data = Vec<u32>;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        Ok(input.lines().map(|x| x.parse().unwrap()).collect())
    }

    fn part1(data: &Self::Data) -> Result<Self::Output> {
        Ok(count_incrementing(data.iter()))
    }

    fn part2(data: &Self::Data) -> Result<Self::Output> {
        Ok(count_incrementing(
            data.windows(3).map(|x| x.iter().sum::<u32>()),
        ))
    }
}
