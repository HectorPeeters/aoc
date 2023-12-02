use crate::error::Result;
use crate::{aoc_test, Solution};

pub struct Solution2021Day6;
aoc_test!(Solution2021Day6, 395_627, 1_767_323_539_209);

impl Solution for Solution2021Day6 {
    const YEAR: u32 = 2021;
    const DAY: u8 = 6;

    type Data = Vec<usize>;
    type Output = u64;

    fn parse(input: &str) -> Result<Self::Data> {
        Ok(input
            .lines()
            .flat_map(|x| x.split(',').map(|y| y.parse().unwrap()).collect::<Vec<_>>())
            .collect())
    }

    fn part1(data: &Self::Data) -> Result<Self::Output> {
        let mut fishes = data.clone();

        for _ in 0..80 {
            let mut new_fishes = vec![];
            for fish in &mut fishes {
                if *fish == 0 {
                    new_fishes.push(8);
                    *fish = 6;
                } else {
                    *fish -= 1;
                }
            }
            fishes.append(&mut new_fishes);
        }

        Ok(fishes.len() as u64)
    }

    fn part2(data: &Self::Data) -> Result<Self::Output> {
        let mut fish_counters = vec![0; 9];

        for fish in data {
            fish_counters[*fish] += 1;
        }

        for _ in 0..256 {
            let dying = fish_counters.remove(0);
            fish_counters.insert(8, dying);
            fish_counters[6] += dying;
        }

        Ok(fish_counters.iter().sum())
    }
}
