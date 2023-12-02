use crate::{aoc_test, Solution};
use crate::{error::Result, read_input_file};
use itertools::Itertools;

#[derive(Default)]
pub struct Solution2021Day8 {
    data: Vec<([Digit; 10], [Digit; 4])>,
}

type Digit = Vec<char>;

impl Solution for Solution2021Day8 {
    fn parse(&mut self) -> Result<()> {
        self.data = read_input_file("src/year2021/day8.txt")?
            .lines()
            .map(|l| {
                let mut parts = l.split(" | ");

                let all_signal = parts.next().unwrap();
                let all_codes = parts.next().unwrap();

                let signals = all_signal
                    .split(' ')
                    .map(|s| s.chars().collect())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                let codes = all_codes
                    .split(' ')
                    .map(|c| c.chars().collect())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                (signals, codes)
            })
            .collect();

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        Ok(self
            .data
            .iter()
            .map(|(_, code)| {
                code.iter()
                    .filter(|c| [2, 3, 4, 7].contains(&c.len()))
                    .count() as u64
            })
            .sum())
    }

    fn part2(&self) -> Result<u64> {
        let segments: [Vec<u8>; 10] = [
            vec![0, 1, 2, 3, 4, 5],    // 0
            vec![1, 2],                // 1
            vec![0, 1, 3, 4, 6],       // 2
            vec![0, 1, 2, 3, 6],       // 3
            vec![1, 2, 5, 6],          // 4
            vec![0, 2, 3, 5, 6],       // 5
            vec![0, 2, 3, 4, 5, 6],    // 6
            vec![0, 1, 2],             // 7
            vec![0, 1, 2, 3, 4, 5, 6], // 8
            vec![0, 1, 2, 3, 5, 6],    // 9
        ];

        for perm in [0, 1, 2, 3, 4, 5, 6].iter().permutations(7) {
            let mut incorrect_permutation = false;

            for (signal, _) in &self.data {
                if incorrect_permutation {
                    break;
                }

                for digit in signal {
                    let mut reindexed = digit
                        .iter()
                        .map(|x| (*x as u8 - b'a'))
                        .map(|x| *perm[x as usize])
                        .collect::<Vec<_>>();

                    //                    println!("{:?} {:?}", digit, reindexed);

                    reindexed.sort_unstable();

                    let number = segments.iter().position(|l| l == &reindexed);
                    match number {
                        None => {
                            incorrect_permutation = true;
                            break;
                        }
                        Some(_digit) => {} //println!("Correct digit {}", digit),
                    }
                }
            }

            if !incorrect_permutation {
                println!("FOUND CORRECT PERMUTATION: {perm:?}");
            }
        }

        Ok(0)
    }
}

aoc_test!(Solution2021Day8, 369, 0);
