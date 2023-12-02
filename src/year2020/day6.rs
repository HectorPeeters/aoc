use std::collections::HashSet;

use crate::{aoc_test, error::Result, Solution};

pub struct Solution2020Day6;
aoc_test!(Solution2020Day6, 6930, 3585);

type Group = Vec<Vec<char>>;

impl Solution for Solution2020Day6 {
    const YEAR: u32 = 2020;
    const DAY: u8 = 6;

    type Data = Vec<Group>;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        Ok(input
            .split("\n\n")
            .map(|group| {
                group
                    .split('\n')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.chars().collect())
                    .collect()
            })
            .collect::<Vec<_>>())
    }

    fn part1(data: &Self::Data) -> Result<Self::Output> {
        let mut result = 0;

        for group in data {
            let mut questions = HashSet::new();

            for person in group {
                for c in person {
                    questions.insert(c);
                }
            }

            result += questions.len() as u32;
        }

        Ok(result)
    }

    fn part2(data: &Self::Data) -> Result<Self::Output> {
        let mut result = 0;

        for group in data {
            let mut questions: HashSet<char> = group[0].iter().copied().collect();

            for person in &group[1..] {
                questions.retain(|c| person.contains(c));
            }

            result += questions.len() as u32;
        }

        Ok(result)
    }
}
