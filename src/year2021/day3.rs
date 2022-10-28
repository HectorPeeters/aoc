use crate::error::Result;
use crate::{aoc_test, read_input_file, Solution};

#[derive(Default)]
pub struct Solution2021Day3 {
    data: Vec<Vec<u8>>,
}

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
    fn parse(&mut self) -> Result<()> {
        self.data = read_input_file("src/year2021/day3.txt")?
            .lines()
            .map(|x| {
                x.chars()
                    .map(|x| Ok(x.to_digit(10).expect("Failed to parse int").try_into()?))
                    .collect()
            })
            .collect::<Result<_>>()?;

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        Ok(cmp!(&self.data, <) * cmp!(&self.data, >))
    }

    fn part2(&self) -> Result<u64> {
        Ok(u64::from(
            first_common(&self.data, |a, b| a >= b) * first_common(&self.data, |a, b| a < b),
        ))
    }
}

aoc_test!(Solution2021Day3, 2_035_764, 2_817_661);
