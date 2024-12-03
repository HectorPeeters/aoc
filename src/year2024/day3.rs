use regex::Regex;

use crate::{aoc_test, error::Result, Solution};

pub struct Solution2024Day3;
aoc_test!(Solution2024Day3, 175_700_056, 71_668_682);

#[derive(Debug, PartialEq)]
#[repr(u16)]
enum OpCode {
    Mul,
    Do,
    Dont,
}

impl TryFrom<&str> for OpCode {
    type Error = ();

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "mul" => Ok(Self::Mul),
            "do" => Ok(Self::Do),
            "don't" => Ok(Self::Dont),
            _ => Err(()),
        }
    }
}

pub struct Instruction {
    op: OpCode,
    args: [u16; 2],
}

impl Solution for Solution2024Day3 {
    const YEAR: u32 = 2024;
    const DAY: u8 = 3;

    type Data = Vec<Instruction>;
    type Output = u32;

    fn parse(input: &str) -> Result<Self::Data> {
        let re = Regex::new(r"([a-z][a-z']+)\((\d+)?(,\d+)*\)").unwrap();

        Ok(re
            .captures_iter(input)
            .filter_map(|c| match OpCode::try_from(c.get(1).unwrap().as_str()) {
                Ok(op) => Some(Instruction {
                    op,
                    args: c
                        .iter()
                        .skip(2)
                        .flatten()
                        .map(|c| c.as_str().trim_start_matches(',').parse::<u16>().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap_or([0; 2]),
                }),
                Err(()) => None,
            })
            .collect())
    }

    fn part1(input: &Self::Data) -> Result<Self::Output> {
        Ok(input
            .iter()
            .filter(|instr| instr.op == OpCode::Mul)
            .map(|instr| instr.args[0] as u32 * instr.args[1] as u32)
            .sum())
    }

    fn part2(input: &Self::Data) -> Result<Self::Output> {
        let mut enabled = true;
        let mut sum = 0;

        for instr in input {
            match instr.op {
                OpCode::Mul if enabled => sum += instr.args[0] as u32 * instr.args[1] as u32,
                OpCode::Do => enabled = true,
                OpCode::Dont => enabled = false,
                OpCode::Mul => {}
            }
        }

        Ok(sum)
    }
}
