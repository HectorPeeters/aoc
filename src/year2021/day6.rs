use crate::error::Result;
use crate::{aoc_test, read_input_file, Solution};

#[derive(Default)]
pub struct Solution2021Day6 {
    data: Vec<usize>,
}

impl Solution for Solution2021Day6 {
    fn parse(&mut self) -> Result<()> {
        self.data = read_input_file("src/year2021/day6.txt")?
            .lines()
            .flat_map(|x| x.split(',').map(|y| y.parse().unwrap()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        let mut fishes = self.data.clone();

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

    fn part2(&self) -> Result<u64> {
        let mut fish_counters = vec![0_u64; 9];

        for fish in &self.data {
            fish_counters[*fish] += 1;
        }

        for _ in 0..256 {
            let dying = fish_counters.remove(0);
            fish_counters.insert(8, dying);
            fish_counters[6] += dying;
        }

        Ok(fish_counters.iter().sum::<u64>())
    }
}

aoc_test!(Solution2021Day6, 395_627, 1_767_323_539_209);
