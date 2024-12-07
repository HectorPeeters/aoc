use itertools::Itertools;

use crate::{aoc_test, error::Result, Solution};

pub struct Solution2024Day7;
aoc_test!(Solution2024Day7, 7_579_994_664_753, 438_027_111_276_610);

impl Solution for Solution2024Day7 {
    const YEAR: u32 = 2024;
    const DAY: u8 = 7;

    type Data = Vec<(u64, Vec<u64>)>;
    type Output = u64;

    fn parse(input: &str) -> Result<Self::Data> {
        Ok(input
            .lines()
            .map(|l| {
                let mut split = l.split(": ");
                let total = split.next().unwrap().parse().unwrap();
                let parts = split
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|p| p.parse().unwrap())
                    .collect();
                (total, parts)
            })
            .collect())
    }

    fn part1(input: &Self::Data) -> Result<Self::Output> {
        let mut result = 0;
        for line in input {
            let (total, parts) = &line;

            let options = 1 << (parts.len() - 1);

            for c in 0..options {
                let mut sum = parts[0];

                for (i, part) in parts.iter().enumerate().skip(1) {
                    match (c >> (i - 1)) & 1 {
                        0 => sum += *part,
                        1 => sum *= *part,
                        _ => unreachable!(),
                    }
                }

                if sum == *total {
                    result += sum;
                    break;
                }
            }
        }

        Ok(result)
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

        let mut result = 0;

        for line in input {
            let (total, parts) = &line;

            for comb in itertools::repeat_n([0, 1, 2], parts.len() - 1).multi_cartesian_product() {
                let mut sum = parts[0];

                for (op, x) in comb.iter().zip(&parts[1..]) {
                    match op {
                        0 => sum += *x,
                        1 => sum *= *x,
                        2 => sum = sum * 10u64.pow(int_log10(*x)) + *x,
                        _ => unreachable!(),
                    }
                }

                if sum == *total {
                    result += total;
                    break;
                }
            }
        }

        Ok(result)
    }
}
