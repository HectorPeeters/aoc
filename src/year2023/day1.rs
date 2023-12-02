use crate::{aoc_test, error::Result, Solution};

pub struct Solution2023Day1;

impl Solution for Solution2023Day1 {
    const YEAR: u32 = 2023;
    const DAY: u8 = 1;

    type Data = Vec<String>;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        Ok(input.lines().map(ToString::to_string).collect())
    }

    fn part1(data: &Self::Data) -> Result<Self::Output> {
        Ok(data
            .iter()
            .map(|line| {
                let mut nums = line.chars().filter_map(|c| c.to_digit(10));
                let first = nums.next().unwrap();
                first * 10 + nums.last().unwrap_or(first)
            })
            .sum::<u32>())
    }

    fn part2(data: &Self::Data) -> Result<Self::Output> {
        const NUMBERS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        Ok(data
            .iter()
            .map(String::as_str)
            .map(|mut line| {
                let mut first = None;
                let mut last = 0;

                while !line.is_empty() {
                    if let Some(digit) = line.chars().next().unwrap().to_digit(10) {
                        first.get_or_insert(digit);
                        last = digit;
                    } else {
                        for (val, num) in NUMBERS.iter().enumerate() {
                            if line.starts_with(num) {
                                last = val as u32 + 1;
                                first.get_or_insert(last);
                                break;
                            }
                        }
                    }

                    line = &line[1..];
                }

                first.unwrap() * 10 + last
            })
            .sum::<u32>())
    }
}

aoc_test!(Solution2023Day1, 54388, 53515);
