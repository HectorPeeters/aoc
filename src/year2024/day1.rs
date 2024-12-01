use itertools::Itertools;

use crate::{aoc_test, Solution};

pub struct Solution2024Day1;
aoc_test!(Solution2024Day1, 2_580_760, 25_358_365);

impl Solution for Solution2024Day1 {
    const YEAR: u32 = 2024;
    const DAY: u8 = 1;

    type Data = (Vec<u32>, Vec<u32>);
    type Output = u32;

    fn parse(input: &str) -> crate::error::Result<Self::Data> {
        let (left, right): (Vec<_>, Vec<_>) = input
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .enumerate()
            .partition(|(i, _)| i % 2 == 0);

        Ok((
            left.into_iter().map(|(_, v)| v).collect(),
            right.into_iter().map(|(_, v)| v).collect(),
        ))
    }

    fn part1(input: &Self::Data) -> crate::error::Result<Self::Output> {
        let (mut left, mut right) = input.clone();
        debug_assert_eq!(left.len(), right.len());

        left.sort_unstable();
        right.sort_unstable();

        Ok(left
            .into_iter()
            .zip(right)
            .map(|(l, r)| l.abs_diff(r))
            .sum())
    }

    fn part2(input: &Self::Data) -> crate::error::Result<Self::Output> {
        let (left, right) = input;
        debug_assert_eq!(left.len(), right.len());

        let right_counts = right.iter().counts();
        Ok(left
            .iter()
            .map(|l| right_counts.get(l).copied().unwrap_or_default() as u32 * l)
            .sum())
    }
}
