use std::{
    fs::File,
    io::{BufReader, *},
};

fn read_input(file: &str) -> Vec<(String, usize)> {
    let file = File::open(file).expect("No input file");
    let buffer = BufReader::new(file);
    buffer
        .lines()
        .map(|x| {
            let parts = x.expect("Faild to read line");
            let collected = parts.split(" ").collect::<Vec<_>>();
            (
                collected[0].to_string(),
                collected[1].parse::<usize>().expect("Failed to parse int"),
            )
        })
        .collect()
}

fn part_one() -> usize {
    let input = read_input("input");

    let mut pos = [0, 0];

    for (instr, value) in &input {
        match &instr[..] {
            "forward" => pos[0] += value,
            "up" => pos[1] -= value,
            "down" => pos[1] += value,
            _ => unreachable!(),
        }
    }

    pos[0] * pos[1]
}

fn part_two() -> usize {
    let input = read_input("input");

    let mut pos = [0, 0, 0]; // horizontal, depth, aim

    for (instr, value) in &input {
        match &instr[..] {
            "forward" => {
                pos[0] += value;
                pos[1] += pos[2] * value;
            }
            "up" => pos[2] -= value,
            "down" => pos[2] += value,
            _ => unreachable!(),
        }
    }

    pos[0] * pos[1]
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
