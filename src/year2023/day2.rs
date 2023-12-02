use crate::{aoc_test, error::Result, read_input_file, Solution};

#[derive(Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => panic!("Invalid color: {s}"),
        }
    }
}

struct Game {
    id: u32,
    cubes: Vec<(u32, Color)>,
}

#[derive(Default)]
pub struct Solution2023Day2 {
    games: Vec<Game>,
}

impl Solution for Solution2023Day2 {
    fn parse(&mut self) -> Result<()> {
        self.games = read_input_file("src/year2023/day2.txt")?
            .lines()
            .map(|line| {
                let (game_str, rounds_str) = line.split_once(':').unwrap();
                let rounds = rounds_str
                    .trim()
                    .split([',', ';'])
                    .map(|cube| {
                        let (count, color) = cube.trim_start().split_once(' ').unwrap();
                        Ok((count.parse::<u32>()?, Color::from(color)))
                    })
                    .collect::<Result<_>>()?;

                Ok(Game {
                    id: game_str.strip_prefix("Game ").unwrap().parse()?,
                    cubes: rounds,
                })
            })
            .collect::<Result<_>>()?;
        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        const AVAILABLE: [u32; 3] = [12, 13, 14];

        Ok(self
            .games
            .iter()
            .filter(|g| {
                g.cubes
                    .iter()
                    .all(|(count, color)| AVAILABLE[*color as usize] >= *count)
            })
            .map(|g| g.id)
            .sum::<u32>() as u64)
    }

    fn part2(&self) -> Result<u64> {
        Ok(self
            .games
            .iter()
            .map(|g| {
                g.cubes
                    .iter()
                    .fold([0, 0, 0], |mut acc, (count, color)| {
                        acc[*color as usize] = acc[*color as usize].max(*count);
                        acc
                    })
                    .iter()
                    .product::<u32>()
            })
            .sum::<u32>() as u64)
    }
}

aoc_test!(Solution2023Day2, 2268, 63542);
