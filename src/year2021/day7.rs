use crate::{aoc_test, Solution};
use crate::{error::Result, read_input_file};

#[derive(Default)]
pub struct Solution2021Day7 {
    data: Vec<i64>,
}

impl Solution for Solution2021Day7 {
    fn parse(&mut self) -> Result<()> {
        self.data = read_input_file("src/year2021/day7.txt")?
            .lines()
            .flat_map(|x| x.split(',').map(|y| y.parse().unwrap()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        let min = *self.data.iter().min().unwrap();
        let max = *self.data.iter().max().unwrap();

        Ok((min..=max)
            .map(|pos| self.data.iter().map(|x| (*x - pos).unsigned_abs()).sum())
            .min()
            .unwrap())
    }

    fn part2(&self) -> Result<u64> {
        let min = *self.data.iter().min().unwrap();
        let max = *self.data.iter().max().unwrap();

        Ok((min..=max)
            .map(|pos| {
                self.data
                    .iter()
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
