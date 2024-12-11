use derive_more::derive::{Add, Display, From, Into, Sub};

use crate::{aoc_test, error::Result, Solution};

pub struct Solution2024Day10;
aoc_test!(Solution2024Day10, 774, 1651);

#[derive(Debug, Display, Clone, Copy, From, Into, Add, Sub, PartialEq, Eq)]
#[display("({_0}, {_1})")]
struct Pos(i32, i32);

impl Pos {
    pub fn to_index(self, width: usize) -> usize {
        (self.0 + self.1 * width as i32) as usize
    }
}

pub struct Data2024Day10 {
    map: Vec<Vec<u32>>,
    width: usize,
    height: usize,
    trail_heads: Vec<Pos>,
    trail_tails: Vec<Pos>,
}

impl Solution for Solution2024Day10 {
    const YEAR: u32 = 2024;
    const DAY: u8 = 10;

    type Data = Data2024Day10;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        let map = input
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| (c as u8 - b'0') as u32).collect())
            .collect::<Vec<Vec<_>>>();

        let width = map[0].len();
        let height = map.len();

        let mut trail_heads = Vec::new();
        let mut trail_tails = Vec::new();

        for (j, row) in map.iter().enumerate() {
            for (i, h) in row.iter().enumerate() {
                let pos = (j as i32, i as i32).into();
                match h {
                    0 => trail_heads.push(pos),
                    9 => trail_tails.push(pos),
                    _ => {}
                }
            }
        }

        Ok(Data2024Day10 {
            map,
            width,
            height,
            trail_heads,
            trail_tails,
        })
    }

    fn part1(input: &Self::Data) -> Result<Self::Output> {
        let mut result = 0;

        for head in &input.trail_heads {
            for tail in &input.trail_tails {
                let mut visited = bit_set::BitSet::with_capacity(input.width * input.height);

                let mut stack = vec![*head];

                while let Some(pos) = stack.pop() {
                    if pos == *tail {
                        result += 1;
                        break;
                    }

                    visited.insert(pos.to_index(input.width));

                    for neighbour in [Pos(0, -1), Pos(1, 0), Pos(0, 1), Pos(-1, 0)] {
                        let next = pos + neighbour;

                        if !(0..input.width as i32).contains(&next.0)
                            || !(0..input.height as i32).contains(&next.1)
                        {
                            continue;
                        }

                        if input.map[next.0 as usize][next.1 as usize]
                            - input.map[pos.0 as usize][pos.1 as usize]
                            != 1
                        {
                            continue;
                        }

                        if visited.contains(next.to_index(input.width)) {
                            continue;
                        }

                        stack.push(next);
                    }
                }
            }
        }

        Ok(result)
    }

    fn part2(input: &Self::Data) -> Result<Self::Output> {
        let mut result = 0;

        for head in &input.trail_heads {
            for tail in &input.trail_tails {
                let mut stack = vec![*head];

                while let Some(pos) = stack.pop() {
                    if pos == *tail {
                        result += 1;
                        continue;
                    }

                    for neighbour in [Pos(0, -1), Pos(1, 0), Pos(0, 1), Pos(-1, 0)] {
                        let next = pos + neighbour;

                        if !(0..input.width as i32).contains(&next.0)
                            || !(0..input.height as i32).contains(&next.1)
                        {
                            continue;
                        }

                        if input.map[next.0 as usize][next.1 as usize]
                            - input.map[pos.0 as usize][pos.1 as usize]
                            != 1
                        {
                            continue;
                        }

                        stack.push(next);
                    }
                }
            }
        }

        Ok(result)
    }
}
