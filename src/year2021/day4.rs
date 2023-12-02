use crate::{aoc_test, error::Result, Solution};

type Board = Vec<Vec<u32>>;

pub struct Solution2021Day4;
aoc_test!(Solution2021Day4, 11536, 1284);

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
    const YEAR: u32 = 2021;
    const DAY: u8 = 4;

    type Data = (Vec<u32>, Vec<Board>);
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        let mut lines = input.lines();

        let draws = lines
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

        Ok((draws, boards))
    }

    fn part1((draws, boards): &Self::Data) -> Result<Self::Output> {
        for i in 1..=draws.len() {
            if let Some(board) = boards.iter().find(|board| check_win(board, &draws[..i])) {
                return Ok(count_score(board, &draws[..i]));
            }
        }

        unreachable!()
    }

    fn part2((draws, boards): &Self::Data) -> Result<Self::Output> {
        for i in (1..draws.len()).rev() {
            if let Some(board) = boards.iter().find(|board| !check_win(board, &draws[..i])) {
                return Ok(count_score(board, &draws[..=i]));
            }
        }

        unreachable!()
    }
}
