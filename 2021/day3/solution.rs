use std::{
    fs::File,
    io::{BufReader, *},
};

fn read_input(file: &str) -> Vec<Vec<u8>> {
    let file = File::open(file).expect("No input file");
    let buffer = BufReader::new(file);
    buffer
        .lines()
        .map(|x| {
            let parts = x.expect("Faild to read line");
            parts
                .chars()
                .map(|x| x.to_digit(10).expect("Failed to parse int") as u8)
                .collect()
        })
        .collect()
}

fn count(input: &[Vec<u8>], value: u8, index: usize) -> usize {
    input
        .iter()
        .map(|x| x[index])
        .filter(|y| *y == value)
        .count()
}

macro_rules! cmp {
    ($input:expr, $op:tt) => {
        (0..$input[0].len())
            .into_iter()
            .map(|i| if count($input, 0, i) $op count($input, 1, i) { 1 << ($input[0].len() - 1 - i) } else { 0 })
            .fold(0, |acc, x| acc | x)
    };
}

fn part_one() -> u32 {
    let input = read_input("input");
    cmp!(&input, <) * cmp!(&input, >)
}

fn first_common(input: &Vec<Vec<u8>>, func: fn(usize, usize) -> bool) -> u32 {
    let mut input = input.clone();
    for i in 0..input[0].len() {
        let most_common_first = func(count(&input, 1, i), count(&input, 0, i)) as u8;

        input = input
            .into_iter()
            .filter(|x| x[i] == most_common_first)
            .collect();

        if input.len() == 1 {
            break;
        }
    }

    input[0].iter().enumerate().fold(0, |acc, (i, value)| {
        acc | ((*value as u32) << (input[0].len() - 1 - i))
    })
}

fn part_two() -> u32 {
    let input = read_input("input");
    first_common(&input, |a, b| a >= b) * first_common(&input, |a, b| a < b)
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
