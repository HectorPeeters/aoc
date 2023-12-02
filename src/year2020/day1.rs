use std::cmp::Ordering;

use crate::{aoc_test, error::Result, read_input_file, Solution};

#[derive(Default)]
pub struct Solution2020Day1 {
    data: Vec<u64>,
}

impl Solution for Solution2020Day1 {
    fn parse(&mut self) -> Result<()> {
        self.data = read_input_file("src/year2020/day1.txt")?
            .lines()
            .map(|x| Ok(x.parse()?))
            .collect::<Result<_>>()?;

        self.data.sort_unstable();

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        let mut low = 0;
        let mut high = self.data.len() - 1;

        loop {
            let sum = self.data[low] + self.data[high];

            match sum.cmp(&2020) {
                Ordering::Equal => break,
                Ordering::Less => low += 1,
                Ordering::Greater => high -= 1,
            }
        }

        Ok(self.data[low] * self.data[high])
    }

    fn part2(&self) -> Result<u64> {
        for value in &self.data {
            let target = 2020 - value;
            let mut low = 0;
            let mut high = self.data.len() - 1;

            loop {
                if low >= high {
                    break;
                }

                let sum = self.data[low] + self.data[high];

                match sum.cmp(&target) {
                    Ordering::Equal => return Ok(value * self.data[low] * self.data[high]),
                    Ordering::Less => low += 1,
                    Ordering::Greater => high -= 1,
                }
            }
        }

        unreachable!()
    }
}

aoc_test!(Solution2020Day1, 964_875, 158_661_360);
