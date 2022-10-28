use crate::{aoc_test, error::Result, read_input_file, Solution};

#[derive(Default)]
pub struct Solution2020Day3 {
    data: Vec<Vec<char>>,
}

impl Solution for Solution2020Day3 {
    fn parse(&mut self) -> Result<()> {
        self.data = read_input_file("src/year2020/day3.txt")?
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        let width = self.data[0].len();

        Ok(self
            .data
            .iter()
            .enumerate()
            .filter(|(y, line)| line[(y * 3) % width] == '#')
            .count() as u64)
    }

    fn part2(&self) -> Result<u64> {
        let width = self.data[0].len();

        let mut sum = 1;

        let x_offset = [1, 3, 5, 7, 1];
        let y_offset = [1, 1, 1, 1, 2];

        for i in 0..5 {
            let mut trees = 0;
            let mut x = 0;
            let mut y = 0;

            loop {
                if self.data[y][x % width] == '#' {
                    trees += 1;
                }

                y += y_offset[i];
                x += x_offset[i];

                if y >= self.data.len() {
                    break;
                }
            }

            sum *= trees;
        }

        Ok(sum)
    }
}

aoc_test!(Solution2020Day3, 176, 5872458240);
