use crate::{aoc_test, error::Result, Solution};

use itertools::Itertools;

pub struct Solution2024Day4;
aoc_test!(Solution2024Day4, 2557, 1854);

pub struct Data2024Day4 {
    chars: Vec<char>,
    cols: usize,
    rows: usize,
}

impl Solution for Solution2024Day4 {
    const YEAR: u32 = 2024;
    const DAY: u8 = 4;

    type Data = Data2024Day4;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().trim().len();
        assert_eq!(rows, cols);

        Ok(Data2024Day4 {
            chars: input.trim().chars().filter(|c| *c != '\n').collect(),
            cols,
            rows,
        })
    }

    fn part1(input: &Self::Data) -> Result<Self::Output> {
        fn search(iter: impl Iterator<Item = char>) -> u32 {
            iter.tuple_windows()
                .filter(|(a, b, c, d)| {
                    let array = [*a, *b, *c, *d];
                    array == ['X', 'M', 'A', 'S'] || array == ['S', 'A', 'M', 'X']
                })
                .count() as u32
        }

        let col = |i: usize| {
            input
                .chars
                .iter()
                .enumerate()
                .filter(move |(j, _)| (j % input.cols) == i)
                .map(|(_, c)| *c)
        };

        let row = |r: usize| {
            input
                .chars
                .iter()
                .enumerate()
                .filter(move |(i, _)| (i / input.cols) == r)
                .map(|(_, c)| *c)
        };

        let diag = |i: usize| {
            input
                .chars
                .iter()
                .enumerate()
                .map(|(i, c)| (i % input.cols, i / input.cols, c))
                .filter(move |(c, r, _)| *c as i32 - *r as i32 == i as i32 - input.cols as i32)
                .map(|(_, _, c)| *c)
        };

        let diag2 = |i: usize| {
            input
                .chars
                .iter()
                .enumerate()
                .map(|(i, c)| (input.cols - i % input.cols - 1, i / input.cols, c))
                .filter(move |(c, r, _)| *c as i32 - *r as i32 == i as i32 - input.cols as i32)
                .map(|(_, _, c)| *c)
        };

        let mut total = 0;

        for i in 0..input.cols {
            total += search(col(i));
        }

        for i in 0..input.rows {
            total += search(row(i));
        }

        for i in 0..input.cols * 2 {
            total += search(diag(i));
            total += search(diag2(i));
        }

        Ok(total)
    }

    fn part2(input: &Self::Data) -> Result<Self::Output> {
        let mut total = 0;

        for y in 1..input.rows - 1 {
            'outer: for x in 1..input.cols - 1 {
                if input.chars[x + y * input.cols] != 'A' {
                    continue;
                }

                let offsets = [[(-1, -1), (1, 1)], [(1, -1), (-1, 1)]];

                for offset in offsets {
                    let mut sum = 0;

                    for (dx, dy) in offset {
                        let x = x as i32 + dx;
                        let y = y as i32 + dy;
                        debug_assert!(x >= 0 && y >= 0);

                        let x = x as usize;
                        let y = y as usize;
                        sum += input.chars[x + y * input.cols] as u8;
                    }

                    if sum != b'S' + b'M' {
                        continue 'outer;
                    }
                }

                total += 1;
            }
        }

        Ok(total)
    }
}
