use crate::error::Result;
use crate::{aoc_test, read_input_file, Solution};
use std::cmp::PartialOrd;

#[derive(Default)]
pub struct Solution2021Day1 {
    data: Vec<u32>,
}

fn count_incrementing<I, T>(data: I) -> u64
where
    T: PartialOrd,
    I: Iterator<Item = T> + Clone,
{
    data.clone()
        .zip(data.skip(1))
        .filter(|(first, second)| first < second)
        .count() as u64
}

impl Solution for Solution2021Day1 {
    fn parse(&mut self) -> Result<()> {
        self.data = read_input_file("src/year2021/day1.txt")?
            .lines()
            .map(|x| x.parse().expect("Failed to parse number"))
            .collect();

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        Ok(count_incrementing(self.data.iter()))
    }

    fn part2(&self) -> Result<u64> {
        Ok(count_incrementing(
            self.data.windows(3).map(|x| x.iter().sum::<u32>()),
        ))
    }
}

aoc_test!(Solution2021Day1, 1185, 1158);
