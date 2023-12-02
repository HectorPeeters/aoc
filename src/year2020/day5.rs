use crate::{aoc_test, error::Result, read_input_file, Solution};

#[derive(Default)]
pub struct Solution2020Day5 {
    data: Vec<String>,
}

impl Solution for Solution2020Day5 {
    fn parse(&mut self) -> Result<()> {
        self.data = read_input_file("src/year2020/day5.txt")?
            .lines()
            .map(ToOwned::to_owned)
            .collect();

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        let mut highest = 0;

        for line in &self.data {
            let chars = &mut line.chars();

            let row_string = chars
                .take(7)
                .collect::<String>()
                .replace('F', "0")
                .replace('B', "1");
            let row = u64::from_str_radix(&row_string, 2)?;

            let col_string = chars
                .take(7)
                .collect::<String>()
                .replace('L', "0")
                .replace('R', "1");
            let col = u64::from_str_radix(&col_string, 2)?;

            let id = row * 8 + col;

            highest = highest.max(id);
        }

        Ok(highest)
    }

    fn part2(&self) -> Result<u64> {
        let mut ids = vec![];

        for line in &self.data {
            let chars = &mut line.chars();

            let row_string = chars
                .take(7)
                .collect::<String>()
                .replace('F', "0")
                .replace('B', "1");
            let row = u64::from_str_radix(&row_string, 2)?;

            let col_string = chars
                .take(7)
                .collect::<String>()
                .replace('L', "0")
                .replace('R', "1");
            let col = u64::from_str_radix(&col_string, 2)?;

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

aoc_test!(Solution2020Day5, 938, 696);
