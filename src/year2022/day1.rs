use itertools::Itertools;

use crate::error::Result;
use crate::{aoc_test, read_input_file, Solution};

#[derive(Default)]
pub struct Solution2022Day1 {
    data: Vec<Vec<u64>>,
}

impl Solution for Solution2022Day1 {
    fn parse(&mut self) -> Result<()> {
        self.data = read_input_file("src/year2022/day1.txt")?
            .split("\n\n")
            .map(|x| x.split('\n').filter_map(|y| y.parse().ok()).collect())
            .collect::<Vec<Vec<u64>>>();

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        Ok(self.data.iter().map(|x| x.iter().sum()).max().unwrap())
    }

    fn part2(&self) -> Result<u64> {
        Ok(self
            .data
            .iter()
            .map(|x| x.iter().sum::<u64>())
            .sorted_unstable()
            .rev()
            .take(3)
            .sum())
    }
}

aoc_test!(Solution2022Day1, 74198, 209_914);
