use std::cmp::Ordering;

use crate::{aoc_test, error::Result, Solution};

pub struct Solution2020Day1;
aoc_test!(Solution2020Day1, 964_875, 158_661_360);

impl Solution for Solution2020Day1 {
    const YEAR: u32 = 2020;
    const DAY: u8 = 1;

    type Data = Vec<u32>;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        let mut data = input
            .lines()
            .map(|x| Ok(x.parse()?))
            .collect::<Result<Vec<_>>>()?;

        data.sort_unstable();

        Ok(data)
    }

    fn part1(input: &Self::Data) -> Result<Self::Output> {
        let mut low = 0;
        let mut high = input.len() - 1;

        loop {
            let sum = input[low] + input[high];

            match sum.cmp(&2020) {
                Ordering::Equal => break,
                Ordering::Less => low += 1,
                Ordering::Greater => high -= 1,
            }
        }

        Ok(input[low] * input[high])
    }

    fn part2(input: &Self::Data) -> Result<Self::Output> {
        for value in input {
            let target = 2020 - value;
            let mut low = 0;
            let mut high = input.len() - 1;

            loop {
                if low >= high {
                    break;
                }

                let sum = input[low] + input[high];

                match sum.cmp(&target) {
                    Ordering::Equal => return Ok(value * input[low] * input[high]),
                    Ordering::Less => low += 1,
                    Ordering::Greater => high -= 1,
                }
            }
        }

        unreachable!()
    }
}
