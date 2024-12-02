use itertools::Itertools;

use crate::{aoc_test, error::Result, Solution};

pub struct Solution2024Day2;
aoc_test!(Solution2024Day2, 269, 337);

#[allow(clippy::cast_possible_wrap)]
fn safe(report: impl Iterator<Item = u32> + Clone) -> bool {
    let decreasing = report
        .clone()
        .tuple_windows()
        .all(|(a, b)| (1..=3).contains(&(a as i32 - b as i32)));
    if decreasing {
        return true;
    }

    let increasing = report
        .tuple_windows()
        .all(|(a, b)| (1..=3).contains(&(b as i32 - a as i32)));

    increasing || decreasing
}

impl Solution for Solution2024Day2 {
    const YEAR: u32 = 2024;
    const DAY: u8 = 2;

    type Data = Vec<Vec<u32>>;
    type Output = usize;

    fn parse(input: &str) -> Result<Self::Data> {
        Ok(input
            .lines()
            .map(|l| l.split_whitespace().map(|p| p.parse().unwrap()).collect())
            .collect())
    }

    fn part1(input: &Self::Data) -> Result<Self::Output> {
        Ok(input.iter().filter(|r| safe(r.iter().copied())).count())
    }

    fn part2(input: &Self::Data) -> Result<Self::Output> {
        Ok(input
            .iter()
            .filter(|report| {
                (0..report.len()).any(|i| {
                    let report = report
                        .iter()
                        .enumerate()
                        .filter_map(|(j, r)| (j != i).then_some(r))
                        .copied();

                    safe(report)
                })
            })
            .count())
    }
}
