use crate::{aoc_test, error::Result, Solution};

pub struct Solution2021Day7;

impl Solution for Solution2021Day7 {
    const YEAR: u32 = 2021;
    const DAY: u8 = 7;

    type Data = Vec<i32>;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        Ok(input
            .lines()
            .flat_map(|x| x.split(',').map(|y| y.parse().unwrap()).collect::<Vec<_>>())
            .collect())
    }

    fn part1(data: &Self::Data) -> Result<Self::Output> {
        let min = *data.iter().min().unwrap();
        let max = *data.iter().max().unwrap();

        Ok((min..=max)
            .map(|pos| data.iter().map(|x| (*x - pos).unsigned_abs()).sum())
            .min()
            .unwrap())
    }

    fn part2(data: &Self::Data) -> Result<Self::Output> {
        let min = *data.iter().min().unwrap();
        let max = *data.iter().max().unwrap();

        Ok((min..=max)
            .map(|pos| {
                data.iter()
                    .map(|x| {
                        let n = (*x - pos).unsigned_abs() + 1;
                        n * (n - 1) / 2
                    })
                    .sum()
            })
            .min()
            .unwrap())
    }
}

aoc_test!(Solution2021Day7, 345_035, 97_038_163);
