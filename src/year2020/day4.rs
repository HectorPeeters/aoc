use crate::{aoc_test, error::Result, Solution};

pub struct Solution2020Day4;
aoc_test!(Solution2020Day4, 208, 167);

impl Solution for Solution2020Day4 {
    const YEAR: u32 = 2020;
    const DAY: u8 = 4;

    type Data = Vec<Vec<(String, String)>>;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        Ok(input
            .split("\n\n")
            .filter(|x| !x.is_empty())
            .map(|passport| {
                passport
                    .split([' ', '\n'])
                    .filter(|x| !x.is_empty())
                    .map(|e| {
                        let mut parts = e.split(':');

                        (
                            parts.next().unwrap().to_owned(),
                            parts.next().unwrap().to_owned(),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect())
    }

    fn part1(data: &Self::Data) -> Result<Self::Output> {
        let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        Ok(data
            .iter()
            .filter(|passport| {
                required
                    .iter()
                    .all(|r| passport.iter().any(|(k, _)| k == r))
            })
            .count() as u32)
    }

    fn part2(data: &Self::Data) -> Result<Self::Output> {
        let match_range = |x: &str, l: u32, h: u32| -> Result<bool> {
            let value = x.parse::<u32>()?;
            Ok(value >= l && value <= h)
        };

        let validators: [(_, Box<dyn Fn(_) -> _>); 7] = [
            ("byr", Box::new(|x| match_range(x, 1920, 2002))),
            ("iyr", Box::new(|x| match_range(x, 2010, 2020))),
            ("eyr", Box::new(|x| match_range(x, 2020, 2030))),
            (
                "hgt",
                Box::new(|x| {
                    if x.ends_with("cm") {
                        match_range(&x.replace("cm", ""), 150, 193)
                    } else if x.ends_with("in") {
                        match_range(&x.replace("in", ""), 59, 76)
                    } else {
                        Ok(false)
                    }
                }),
            ),
            (
                "hcl",
                Box::new(|x| {
                    Ok(x.len() == 7
                        && x.starts_with('#')
                        && x.chars().skip(1).all(char::is_alphanumeric))
                }),
            ),
            (
                "ecl",
                Box::new(|x| Ok(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&x))),
            ),
            (
                "pid",
                Box::new(|x| Ok(x.len() == 9 && x.chars().all(char::is_numeric))),
            ),
        ];

        Ok(data
            .iter()
            .filter(|passport| {
                validators.iter().all(|(name, callback)| {
                    passport
                        .iter()
                        .any(|(k, v)| k == name && callback(v).unwrap())
                })
            })
            .count() as u32)
    }
}
