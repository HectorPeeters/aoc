use crate::error::Result;
use crate::{aoc_test, read_input_file, Solution};
use std::cmp::{max, min};
use std::collections::HashMap;

type Point = (isize, isize);

#[derive(Default)]
pub struct Solution2021Day5 {
    data: Vec<(Point, Point)>,
}

impl Solution for Solution2021Day5 {
    fn parse(&mut self) -> Result<()> {
        self.data = read_input_file("src/year2021/day5.txt")?
            .lines()
            .map(|line| {
                let (first, second) = line.split_once(" -> ").unwrap();

                let start: Vec<_> = first
                    .split(',')
                    .map(|x| Ok(x.parse::<isize>()?))
                    .collect::<Result<_>>()?;

                let end: Vec<_> = second
                    .split(',')
                    .map(|x| Ok(x.parse()?))
                    .collect::<Result<_>>()?;

                Ok(((start[0], start[1]), (end[0], end[1])))
            })
            .collect::<Result<_>>()?;

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        let mut map: HashMap<Point, isize> = HashMap::new();

        self.data
            .iter()
            .flat_map(|(start, end)| {
                let mut line_points: Vec<Point> = vec![];

                if start.0 == end.0 {
                    for y in min(start.1, end.1)..=max(start.1, end.1) {
                        line_points.push((start.0, y));
                    }
                } else if start.1 == end.1 {
                    for x in min(start.0, end.0)..=max(start.0, end.0) {
                        line_points.push((x, start.1));
                    }
                }

                line_points
            })
            .for_each(|p| {
                *map.entry(p).or_insert(0) += 1;
            });

        Ok(map.iter().filter(|(_, v)| **v > 1).count() as u64)
    }

    fn part2(&self) -> Result<u64> {
        let points = self
            .data
            .iter()
            .flat_map(|(start, end)| {
                let mut line_points = vec![];
                let dx = end.0 - start.0;
                let dy = end.1 - start.1;

                if dx == 0 {
                    for y in min(start.1, end.1)..=max(start.1, end.1) {
                        line_points.push((start.0, y));
                    }
                } else if dy == 0 {
                    for x in min(start.0, end.0)..=max(start.0, end.0) {
                        line_points.push((x, start.1));
                    }
                } else {
                    assert_eq!(dx.abs(), dy.abs());
                    for i in 0..=dx.abs() {
                        line_points.push((start.0 + dx.signum() * i, start.1 + dy.signum() * i));
                    }
                }
                line_points
            })
            .collect::<Vec<_>>();

        let mut map: HashMap<Point, isize> = HashMap::new();

        for p in &points {
            *map.entry(*p).or_insert(0) += 1;
        }

        Ok(map.iter().filter(|(_, v)| **v > 1).count() as u64)
    }
}

aoc_test!(Solution2021Day5, 7468, 22364);
