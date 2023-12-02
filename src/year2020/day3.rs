use crate::{aoc_test, error::Result, Solution};

pub struct Solution2020Day3;
aoc_test!(Solution2020Day3, 176, 5_872_458_240);

impl Solution for Solution2020Day3 {
    const YEAR: u32 = 2020;
    const DAY: u8 = 3;

    type Data = Vec<Vec<char>>;
    type Output = u64;

    fn parse(input: &str) -> Result<Self::Data> {
        Ok(input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect())
    }

    fn part1(data: &Self::Data) -> Result<Self::Output> {
        let width = data[0].len();

        Ok(data
            .iter()
            .enumerate()
            .filter(|(y, line)| line[(y * 3) % width] == '#')
            .count() as u64)
    }

    fn part2(data: &Self::Data) -> Result<Self::Output> {
        let width = data[0].len();

        let mut sum = 1;

        let x_offset = [1, 3, 5, 7, 1];
        let y_offset = [1, 1, 1, 1, 2];

        for i in 0..5 {
            let mut trees = 0;
            let mut x = 0;
            let mut y = 0;

            loop {
                if data[y][x % width] == '#' {
                    trees += 1;
                }

                y += y_offset[i];
                x += x_offset[i];

                if y >= data.len() {
                    break;
                }
            }

            sum *= trees;
        }

        Ok(sum)
    }
}
