use crate::{aoc_test, error::Result, Solution};

pub struct Solution2020Day2;
aoc_test!(Solution2020Day2, 396, 428);

type Range = (usize, usize);

impl Solution for Solution2020Day2 {
    const YEAR: u32 = 2020;
    const DAY: u8 = 2;

    type Data = Vec<(Range, char, String)>;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        input
            .lines()
            .map(|line| {
                let mut parts = line.split(' ');

                let mut range_parts = parts.next().unwrap().split('-');
                let range_min: usize = range_parts.next().unwrap().parse()?;
                let range_max: usize = range_parts.next().unwrap().parse()?;

                let letter = parts.next().unwrap().chars().next().unwrap();
                let password = parts.next().unwrap();

                Ok(((range_min, range_max), letter, password.to_string()))
            })
            .collect::<Result<_>>()
    }

    fn part1(data: &Self::Data) -> Result<Self::Output> {
        Ok(data
            .iter()
            .filter(|((min, max), letter, password)| {
                let count = password.chars().filter(|c| c == letter).count();
                (min..=max).contains(&&count)
            })
            .count() as u32)
    }

    fn part2(data: &Self::Data) -> Result<Self::Output> {
        Ok(data
            .iter()
            .filter(|((a, b), letter, password)| {
                (password.chars().nth(*a - 1).unwrap() == *letter)
                    != (password.chars().nth(*b - 1).unwrap() == *letter)
            })
            .count() as u32)
    }
}
