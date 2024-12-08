use std::collections::{HashMap, HashSet};

use derive_more::{
    derive::{Mul, Sub},
    Add, From, Into,
};
use itertools::Itertools;

use crate::{aoc_test, error::Result, Solution};

pub struct Solution2024Day8;
aoc_test!(Solution2024Day8, 0, 0);

#[derive(Debug, Clone, Copy, From, Into, Add, Sub, Mul, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

pub struct Data2024Day8 {
    antennas: HashMap<char, Vec<Pos>>,
    width: u32,
    height: u32,
}

impl Solution for Solution2024Day8 {
    const YEAR: u32 = 2024;
    const DAY: u8 = 8;

    type Data = Data2024Day8;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        let mut antennas = HashMap::new();
        let width = input.lines().next().unwrap().len() as u32;
        let height = input.lines().count() as u32;

        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
                let pos = (j as i32, i as i32).into();
                antennas
                    .entry(c)
                    .and_modify(|e: &mut Vec<_>| e.push(pos))
                    .or_insert(vec![pos]);
            }
        }

        Ok(Data2024Day8 {
            antennas,
            width,
            height,
        })
    }

    fn part1(input: &Self::Data) -> Result<Self::Output> {
        Ok(input
            .antennas
            .values()
            .flat_map(|a| {
                a.iter()
                    .tuple_combinations()
                    .flat_map(|(a, b)| [*b + (*b - *a), *a + (*a - *b)])
                    .filter(|pos| {
                        pos.0 >= 0
                            && pos.0 < input.width as i32
                            && pos.1 >= 0
                            && pos.1 < input.height as i32
                    })
            })
            .collect::<HashSet<_>>()
            .len() as u32)
    }

    fn part2(input: &Self::Data) -> Result<Self::Output> {
        let mut set = HashSet::<Pos>::new();
        for antennas in input.antennas.values() {
            for (a, b) in antennas.iter().tuple_combinations() {
                #[allow(clippy::maybe_infinite_iter)]
                (0..)
                    .flat_map(|i| [*b + (*b - *a) * i, *a + (*a - *b) * i])
                    .take_while(|pos| {
                        pos.0 >= 0
                            && pos.0 < input.width as i32
                            && pos.1 >= 0
                            && pos.1 < input.height as i32
                    })
                    .for_each(|pos| {
                        set.insert(pos);
                    });
            }
        }

        Ok(set.len() as u32)
    }
}
