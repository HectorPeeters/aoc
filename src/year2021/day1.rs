use crate::error::Result;
use crate::{aoc_test, read_input_file, Solution};
use std::cmp::PartialOrd;

#[derive(Default)]
pub struct Solution2021Day1 {
    data: Vec<u32>,
}

fn count_incrementing<T: PartialOrd>(data: &[T]) -> usize {
    data.iter()
        .zip(&data[1..])
        .filter(|(first, second)| first < second)
        .count()
}

impl Solution for Solution2021Day1 {
    fn parse(&mut self) -> Result<()> {
        let data = read_input_file("src/year2021/day1.txt")?
            .lines()
            .map(|x| x.parse().expect("Failed to parse number"))
            .collect();

        self.data = data;

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        Ok(count_incrementing(&self.data) as u64)
    }

    fn part2(&self) -> Result<u64> {
        Ok(count_incrementing(
            &self
                .data
                .windows(3)
                .map(|x| x.iter().sum())
                .collect::<Vec<u32>>(),
        ) as u64)
    }
}

aoc_test!(Solution2021Day1, 1185, 1158);
