use std::cmp::Ordering;

use itertools::Itertools;

use crate::{aoc_test, error::Result, Solution};

pub struct Solution2024Day5;
aoc_test!(Solution2024Day5, 5762, 4130);

pub struct Data2024Day5 {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

impl Solution for Solution2024Day5 {
    const YEAR: u32 = 2024;
    const DAY: u8 = 5;

    type Data = Data2024Day5;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        let (rules, updates) = input.split_once("\n\n").unwrap();

        let rules = rules
            .lines()
            .map(|l| {
                l.split('|')
                    .map(|p| p.parse().unwrap())
                    .next_tuple()
                    .unwrap()
            })
            .collect();

        let updates = updates
            .lines()
            .map(|l| l.split(',').map(|p| p.parse().unwrap()).collect())
            .collect();

        Ok(Data2024Day5 { rules, updates })
    }

    fn part1(input: &Self::Data) -> Result<Self::Output> {
        let mut result = 0;

        for update in &input.updates {
            let find_rule = |a, b| input.rules.iter().find(|(ar, br)| *ar == a && *br == b);

            let valid = update.is_sorted_by(|a, b| match (find_rule(*a, *b), find_rule(*b, *a)) {
                (_, None) => true,
                (None, Some(_)) => false,
                (Some(_), Some(_)) => unreachable!(),
            });

            if valid {
                result += update[update.len() / 2];
            }
        }

        Ok(result)
    }

    fn part2(input: &Self::Data) -> Result<Self::Output> {
        let mut result = 0;

        for update in &input.updates {
            let find_rule = |a, b| input.rules.iter().find(|(ar, br)| *ar == a && *br == b);

            let mut sorted_update = update.clone();
            sorted_update.sort_by(|a, b| match (find_rule(*a, *b), find_rule(*b, *a)) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(_), Some(_)) => unreachable!(),
            });

            if sorted_update != *update {
                result += sorted_update[sorted_update.len() / 2];
            }
        }

        Ok(result)
    }
}
