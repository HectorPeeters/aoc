use crate::error::Result;
use crate::{aoc_test, Solution};

pub struct Solution2021Day3;
aoc_test!(Solution2021Day3, 2_035_764, 2_817_661);

fn count(input: &[Vec<u8>], value: u8, index: usize) -> usize {
    input
        .iter()
        .map(|x| x[index])
        .filter(|y| *y == value)
        .count()
}

macro_rules! cmp {
    ($input:expr, $op:tt) => {
        (0..$input[0].len())
            .into_iter()
            .map(|i|
                if count($input, 0, i) $op count($input, 1, i) {
                    1 << ($input[0].len() - 1 - i)
                } else {
                    0
                })
            .fold(0, |acc, x| acc | x)
    };
}

fn first_common(input: &[Vec<u8>], func: fn(usize, usize) -> bool) -> u32 {
    let mut input = input.to_owned();
    for i in 0..input[0].len() {
        let most_common_first = u8::from(func(count(&input, 1, i), count(&input, 0, i)));

        input.retain(|x| x[i] == most_common_first);

        if input.len() == 1 {
            break;
        }
    }

    input[0].iter().enumerate().fold(0, |acc, (i, value)| {
        acc | (u32::from(*value) << (input[0].len() - 1 - i))
    })
}

impl Solution for Solution2021Day3 {
    const YEAR: u32 = 2021;
    const DAY: u8 = 3;

    type Data = Vec<Vec<u8>>;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        input
            .lines()
            .map(|x| {
                x.chars()
                    .map(|x| Ok(x.to_digit(10).expect("Failed to parse int").try_into()?))
                    .collect()
            })
            .collect()
    }

    fn part1(data: &Self::Data) -> Result<Self::Output> {
        Ok(cmp!(data, <) * cmp!(data, >))
    }

    fn part2(data: &Self::Data) -> Result<Self::Output> {
        Ok(u32::from(
            first_common(data, |a, b| a >= b) * first_common(data, |a, b| a < b),
        ))
    }
}
