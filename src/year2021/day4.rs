use crate::error::Result;
use crate::{aoc_test, read_input_file, Solution};

type Board = Vec<Vec<u32>>;

#[derive(Default)]
pub struct Solution2021Day4 {
    draws: Vec<u32>,
    boards: Vec<Board>,
}

fn check_win(board: &[Vec<u32>], draws: &[u32]) -> bool {
    board.iter().any(|x| x.iter().all(|y| draws.contains(y)))
        || (0..board.len()).any(|x| board.iter().all(|y| draws.contains(&y[x])))
}

fn count_score(board: &[Vec<u32>], draws: &[u32]) -> u32 {
    board
        .iter()
        .flatten()
        .filter(|x| !draws.contains(x))
        .sum::<u32>()
        * *draws.last().unwrap()
}

impl Solution for Solution2021Day4 {
    fn parse(&mut self) -> Result<()> {
        let input = read_input_file("src/year2021/day4.txt")?;
        let mut lines = input.lines();

        self.draws = lines
            .next()
            .unwrap()
            .split(',')
            .map(|x| Ok(x.parse()?))
            .collect::<Result<_>>()?;

        let mut boards: Vec<Board> = vec![vec![]];

        for line in lines.skip(1) {
            if line.is_empty() {
                boards.push(vec![]);
            }

            let values = line
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| Ok(x.parse()?))
                .collect::<Result<Vec<_>>>()?;

            if !values.is_empty() {
                if let Some(last) = boards.last_mut() {
                    last.push(values);
                }
            }
        }

        self.boards = boards;

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        for i in 1..=self.draws.len() {
            if let Some(board) = self
                .boards
                .iter()
                .find(|board| check_win(board, &self.draws[..i]))
            {
                return Ok(u64::from(count_score(board, &self.draws[..i])));
            }
        }

        unreachable!()
    }

    fn part2(&self) -> Result<u64> {
        for i in (1..self.draws.len()).rev() {
            if let Some(board) = self
                .boards
                .iter()
                .find(|board| !check_win(board, &self.draws[..i]))
            {
                return Ok(u64::from(count_score(board, &self.draws[..=i])));
            }
        }

        unreachable!()
    }
}

aoc_test!(Solution2021Day4, 11536, 1284);
