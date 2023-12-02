use crate::{aoc_test, error::Result, Solution};

pub struct Solution2020Day5;
aoc_test!(Solution2020Day5, 938, 696);

impl Solution for Solution2020Day5 {
    const YEAR: u32 = 2020;
    const DAY: u8 = 5;

    type Data = Vec<String>;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        Ok(input.lines().map(ToOwned::to_owned).collect())
    }

    fn part1(data: &Self::Data) -> Result<Self::Output> {
        let mut highest = 0;

        for line in data {
            let chars = &mut line.chars();

            let row_string = chars
                .take(7)
                .collect::<String>()
                .replace('F', "0")
                .replace('B', "1");
            let row = u32::from_str_radix(&row_string, 2)?;

            let col_string = chars
                .take(7)
                .collect::<String>()
                .replace('L', "0")
                .replace('R', "1");
            let col = u32::from_str_radix(&col_string, 2)?;

            let id = row * 8 + col;

            highest = highest.max(id);
        }

        Ok(highest)
    }

    fn part2(data: &Self::Data) -> Result<Self::Output> {
        let mut ids = vec![];

        for line in data {
            let chars = &mut line.chars();

            let row_string = chars
                .take(7)
                .collect::<String>()
                .replace('F', "0")
                .replace('B', "1");
            let row = u32::from_str_radix(&row_string, 2)?;

            let col_string = chars
                .take(7)
                .collect::<String>()
                .replace('L', "0")
                .replace('R', "1");
            let col = u32::from_str_radix(&col_string, 2)?;

            ids.push(row * 8 + col);
        }

        ids.sort_unstable();

        for i in 0..ids.len() - 1 {
            if ids[i] != ids[i + 1] - 1 {
                return Ok((ids[i] + ids[i + 1]) / 2);
            }
        }

        unreachable!()
    }
}
