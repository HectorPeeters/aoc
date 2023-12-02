use crate::{aoc_test, error::Result, read_input_file, Solution};

#[derive(Default)]
pub struct Solution2023Day1 {
    lines: Vec<String>,
}

impl Solution for Solution2023Day1 {
    fn parse(&mut self) -> Result<()> {
        self.lines = read_input_file("src/year2023/day1.txt")?
            .lines()
            .map(ToString::to_string)
            .collect();
        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        Ok(self
            .lines
            .iter()
            .map(|line| {
                let mut nums = line.chars().filter_map(|c| c.to_digit(10));
                let first = nums.next().unwrap();
                first * 10 + nums.last().unwrap_or(first)
            })
            .sum::<u32>() as u64)
    }

    fn part2(&self) -> Result<u64> {
        const NUMBERS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        Ok(self
            .lines
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
            .sum::<u32>() as u64)
    }
}

aoc_test!(Solution2023Day1, 54388, 53515);
