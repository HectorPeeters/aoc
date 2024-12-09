use std::num::NonZeroU32;

use crate::{aoc_test, error::Result, Solution};

pub struct Solution2024Day9;
aoc_test!(Solution2024Day9, 6_262_891_638_328, 6_287_317_016_845);

impl Solution for Solution2024Day9 {
    const YEAR: u32 = 2024;
    const DAY: u8 = 9;

    type Data = Vec<Option<NonZeroU32>>;
    type Output = u64;

    fn parse(input: &str) -> Result<Self::Data> {
        let input = input.trim();
        let mut result = Vec::with_capacity(input.len());

        let mut id = 1;
        for (i, c) in input.chars().enumerate() {
            let count = c as u8 - b'0';

            if i % 2 == 0 {
                for _ in 0..count {
                    result.push(Some(NonZeroU32::new(id).unwrap()));
                }
                id += 1;
            } else {
                for _ in 0..count {
                    result.push(None);
                }
            }
        }

        Ok(result)
    }

    fn part1(input: &Self::Data) -> Result<Self::Output> {
        let mut input = input.clone();

        let mut a = 0;
        let mut b = input.len() - 1;

        loop {
            a = match input.iter().skip(a).position(Option::is_none) {
                Some(x) => a + x,
                None => break,
            };
            b = match input
                .iter()
                .rev()
                .skip(input.len() - 1 - b)
                .position(Option::is_some)
            {
                Some(x) => b - x,
                None => break,
            };

            if b <= a {
                break;
            }

            input.swap(a, b);
        }

        Ok(input
            .into_iter()
            .map_while(|x| x)
            .enumerate()
            .map(|(pos, i)| pos as u64 * (i.get() - 1) as u64)
            .sum())
    }

    fn part2(input: &Self::Data) -> Result<Self::Output> {
        let mut input = input.clone();

        let mut free_space = Vec::new();
        let mut start = None;

        for (i, x) in input.iter().enumerate() {
            match (start, x) {
                (None, None) => start = Some(i),
                (Some(s), Some(_)) => {
                    free_space.push((s, i - s));
                    start = None;
                }
                _ => {}
            }
        }

        let mut end = input.len();

        loop {
            let mut id = None;
            let mut start = 0;
            let mut len = 0;

            if end == 0 {
                break;
            }

            for i in (0..end).rev() {
                let Some(d) = input[i] else {
                    continue;
                };
                if id.is_some() && id.unwrap() != d.get() {
                    break;
                }
                id = Some(d.get());
                len += 1;
                start = i;
            }

            end = start;

            let Some(space_index) = free_space.iter().position(|s| s.1 >= len) else {
                continue;
            };

            let space = &mut free_space[space_index];
            let space_start = space.0;

            if space_start >= start {
                continue;
            }

            space.0 += len;
            space.1 -= len;

            for i in 0..len {
                input.swap(space_start + i, start + i);
            }

            // for i in &input {
            //     match i {
            //         Some(i) => print!("{}", i.get() - 1),
            //         None => print!("."),
            //     }
            // }
            // println!();
        }

        Ok(input
            .into_iter()
            .enumerate()
            .filter_map(|(pos, x)| x.map(|x| (pos, x)))
            .map(|(pos, i)| pos as u64 * (i.get() - 1) as u64)
            .sum())
    }
}
