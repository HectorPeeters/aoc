use std::collections::HashMap;

use crate::{aoc_test, error::Result, Solution};

pub struct Solution2024Day11;
aoc_test!(Solution2024Day11, 200_446, 238_317_474_993_392);

impl Solution for Solution2024Day11 {
    const YEAR: u32 = 2024;
    const DAY: u8 = 11;

    type Data = Vec<u64>;
    type Output = u64;

    fn parse(input: &str) -> Result<Self::Data> {
        Ok(input
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect())
    }

    fn part1(input: &Self::Data) -> Result<Self::Output> {
        fn int_log10(mut i: u64) -> u32 {
            let mut len = 0;
            while i > 0 {
                i /= 10;
                len += 1;
            }
            len
        }

        let mut input = input.clone();

        for _ in 0..25 {
            let mut i = 0;
            while i < input.len() {
                let value = &mut input[i];

                if *value == 0 {
                    *value = 1;
                    i += 1;
                    continue;
                }

                let digits = int_log10(*value);
                if digits % 2 == 0 {
                    let divisor = 10_u64.pow(digits / 2);

                    let left_value = *value / divisor;
                    let right_value = *value % divisor;

                    *value = left_value;
                    input.insert(i + 1, right_value);

                    i += 2;
                    continue;
                }

                *value *= 2024;
                i += 1;
            }
        }

        Ok(input.len() as u64)
    }

    fn part2(input: &Self::Data) -> Result<Self::Output> {
        fn int_log10(mut i: u64) -> u32 {
            let mut len = 0;
            while i > 0 {
                i /= 10;
                len += 1;
            }
            len
        }

        let mut values = Vec::<(u64, u64)>::new();

        for i in input {
            values.push((*i, 1));
        }

        let mut new_values = Vec::new();

        for _ in 0..75 {
            for (value, count) in &mut values {
                if *value == 0 {
                    new_values.push((1, *count));
                    continue;
                }

                let digits = int_log10(*value);
                if digits % 2 == 0 {
                    let divisor = 10_u64.pow(digits / 2);

                    new_values.push((*value / divisor, *count));
                    new_values.push((*value % divisor, *count));
                    continue;
                }

                new_values.push((*value * 2024, *count));
            }

            let mut map = HashMap::new();

            for (id, count) in &new_values {
                *map.entry(*id).or_insert(0) += count;
            }

            values = map.into_iter().collect();
            new_values.clear();
        }

        Ok(values.iter().map(|(_, c)| c).sum())
    }
}
