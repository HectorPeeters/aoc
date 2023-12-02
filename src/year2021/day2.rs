use crate::error::Result;
use crate::{aoc_test, Solution};

pub struct Solution2021Day2;
aoc_test!(Solution2021Day2, 1_604_850, 1_685_186_100);

impl Solution for Solution2021Day2 {
    const YEAR: u32 = 2021;
    const DAY: u8 = 2;

    type Data = Vec<(String, u32)>;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        input
            .lines()
            .map(|x| {
                let mut parts = x.split(' ');
                Ok((
                    parts.next().unwrap().to_owned(),
                    parts.next().unwrap().parse()?,
                ))
            })
            .collect()
    }

    fn part1(data: &Self::Data) -> Result<Self::Output> {
        let mut pos = [0, 0];

        for (instr, value) in data {
            match &instr[..] {
                "forward" => pos[0] += value,
                "up" => pos[1] -= value,
                "down" => pos[1] += value,
                _ => unreachable!(),
            }
        }

        Ok(u32::from(pos[0] * pos[1]))
    }

    fn part2(data: &Self::Data) -> Result<Self::Output> {
        let mut pos = [0, 0, 0]; // horizontal, depth, aim

        for (instr, value) in data {
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

        Ok(u32::from(pos[0] * pos[1]))
    }
}
