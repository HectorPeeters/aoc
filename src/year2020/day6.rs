use std::collections::HashSet;

use crate::{aoc_test, error::Result, read_input_file, Solution};

#[derive(Default)]
pub struct Solution2020Day6 {
    data: Vec<Group>,
}

type Group = Vec<Vec<char>>;

impl Solution for Solution2020Day6 {
    fn parse(&mut self) -> Result<()> {
        self.data = read_input_file("src/year2020/day6.txt")?
            .split("\n\n")
            .map(|group| {
                group
                    .split('\n')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.chars().collect())
                    .collect()
            })
            .collect::<Vec<_>>();

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        let mut result = 0;

        for group in &self.data {
            let mut questions = HashSet::new();

            for person in group {
                for c in person {
                    questions.insert(c);
                }
            }

            result += questions.len() as u64;
        }

        Ok(result)
    }

    fn part2(&self) -> Result<u64> {
        let mut result = 0;

        for group in &self.data {
            let mut questions: HashSet<char> = group[0].iter().copied().collect();

            for person in &group[1..] {
                questions.retain(|c| person.contains(c));
            }

            result += questions.len() as u64;
        }

        Ok(result)
    }
}

aoc_test!(Solution2020Day6, 6930, 3585);
