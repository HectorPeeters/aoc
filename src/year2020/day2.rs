use crate::{aoc_test, error::Result, read_input_file, Solution};

#[derive(Default)]
pub struct Solution2020Day2 {
    data: Vec<(Range, char, String)>,
}

type Range = (usize, usize);

impl Solution for Solution2020Day2 {
    fn parse(&mut self) -> Result<()> {
        self.data = read_input_file("src/year2020/day2.txt")?
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
            .collect::<Result<_>>()?;

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        Ok(self
            .data
            .iter()
            .filter(|((min, max), letter, password)| {
                let count = password.chars().filter(|c| c == letter).count();
                (min..=max).contains(&&count)
            })
            .count() as u64)
    }

    fn part2(&self) -> Result<u64> {
        Ok(self
            .data
            .iter()
            .filter(|((a, b), letter, password)| {
                (password.chars().nth(*a - 1).unwrap() == *letter)
                    != (password.chars().nth(*b - 1).unwrap() == *letter)
            })
            .count() as u64)
    }
}

aoc_test!(Solution2020Day2, 396, 428);
