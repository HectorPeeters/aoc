use bit_set::BitSet;

use crate::{aoc_test, error::Result, Solution};

pub struct Solution2024Day6;
aoc_test!(Solution2024Day6, 5453, 2188);

pub struct Data2024Day6 {
    guard: (i32, i32),
    map: Vec<Vec<bool>>,
    width: u32,
    height: u32,
}

impl Solution for Solution2024Day6 {
    const YEAR: u32 = 2024;
    const DAY: u8 = 6;

    type Data = Data2024Day6;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        let mut map = Vec::new();
        let mut guard = (0, 0);

        for (i, line) in input.lines().enumerate() {
            let mut map_row = Vec::new();

            for (j, c) in line.chars().enumerate() {
                if c == '^' {
                    guard = (i as i32, j as i32);
                }

                map_row.push(c == '#');
            }

            map.push(map_row);
        }

        Ok(Data2024Day6 {
            guard,
            width: map[0].len() as u32,
            height: map.len() as u32,
            map,
        })
    }

    fn part1(input: &Self::Data) -> Result<Self::Output> {
        let offsets = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        let mut direction = 0; // 0: top, 1: right, 2: bottom, 3: left
        let mut guard = input.guard;

        let pos_index = |x: (i32, i32)| (x.0 as u32 * input.height + x.1 as u32) as usize;

        let sense = |guard: (i32, i32), dir: usize| {
            let offset: (i32, i32) = offsets[dir];
            let sense_pos = (guard.0 + offset.0, guard.1 + offset.1);
            if sense_pos.0 < 0
                || sense_pos.1 < 0
                || sense_pos.0 >= input.height as i32
                || sense_pos.1 >= input.width as i32
            {
                None
            } else {
                Some(input.map[sense_pos.0 as usize][sense_pos.1 as usize])
            }
        };

        let mut positions = BitSet::with_capacity((input.width * input.height) as usize);
        positions.insert(pos_index(guard));

        while let Some(sense_result) = sense(guard, direction) {
            if sense_result {
                direction = (direction + 1) % 4;
                continue;
            }

            let offset = offsets[direction];
            guard = (guard.0 + offset.0, guard.1 + offset.1);

            positions.insert(pos_index(guard));
        }

        Ok(positions.len() as u32)
    }

    fn part2(input: &Self::Data) -> Result<Self::Output> {
        let offsets = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        let mut visited: Vec<Vec<u32>> = vec![vec![5; input.height as usize]; input.width as usize];

        let mut loops = 0;

        for x in 0..input.width {
            for y in 0..input.height {
                if input.map[y as usize][x as usize] {
                    continue;
                }

                visited.iter_mut().for_each(|row| row.fill(5));

                let mut direction = 0u32; // 0: top, 1: right, 2: bottom, 3: left
                let mut guard = input.guard;

                let sense = |guard: (i32, i32), dir: u32| {
                    let offset: (i32, i32) = offsets[dir as usize];
                    let sense_pos = (guard.0 + offset.0, guard.1 + offset.1);
                    if sense_pos.0 < 0
                        || sense_pos.1 < 0
                        || sense_pos.0 >= input.height as i32
                        || sense_pos.1 >= input.width as i32
                    {
                        None
                    } else if sense_pos.0 == y as i32 && sense_pos.1 == x as i32 {
                        Some(true)
                    } else {
                        Some(input.map[sense_pos.0 as usize][sense_pos.1 as usize])
                    }
                };

                while let Some(sense_result) = sense(guard, direction) {
                    if sense_result {
                        direction = (direction + 1) % 4;
                        continue;
                    }

                    if visited[guard.0 as usize][guard.1 as usize] == direction {
                        loops += 1;
                        break;
                    }

                    visited[guard.0 as usize][guard.1 as usize] = direction;

                    let offset = offsets[direction as usize];
                    guard = (guard.0 + offset.0, guard.1 + offset.1);
                }
            }
        }

        Ok(loops)
    }
}
