use crate::{aoc_test, Solution};
use crate::{error::Result, read_input_file};

#[derive(Default)]
pub struct Solution2021Day2 {
    data: Vec<(String, u32)>,
}

impl Solution for Solution2021Day2 {
    fn parse(&mut self) -> Result<()> {
        self.data = read_input_file("src/year2021/day2.txt")?
            .lines()
            .map(|x| {
                let collected = x.split(' ').collect::<Vec<_>>();
                (
                    collected[0].to_string(),
                    collected[1].parse().expect("Failed to parse int"),
                )
            })
            .collect();

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        let mut pos = [0, 0];

        for (instr, value) in &self.data {
            match &instr[..] {
                "forward" => pos[0] += value,
                "up" => pos[1] -= value,
                "down" => pos[1] += value,
                _ => unreachable!(),
            }
        }

        Ok(u64::from(pos[0] * pos[1]))
    }

    fn part2(&self) -> Result<u64> {
        let mut pos = [0, 0, 0]; // horizontal, depth, aim

        for (instr, value) in &self.data {
            match &instr[..] {
                "forward" => {
                    pos[0] += value;
                    pos[1] += pos[2] * value;
                }
                "up" => pos[2] -= value,
                "down" => pos[2] += value,
                _ => unreachable!(),
            }
        }

        Ok(u64::from(pos[0] * pos[1]))
    }
}

aoc_test!(Solution2021Day2, 1_604_850, 1_685_186_100);
