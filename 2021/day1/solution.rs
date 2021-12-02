use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::str::FromStr;
use std::{
    fs::File,
    io::{BufReader, *},
};

fn read_input<T: FromStr>(file: &str) -> Vec<T>
where
    T::Err: Debug,
{
    let file = File::open(file).expect("No input file");
    let buffer = BufReader::new(file);
    buffer
        .lines()
        .map(|x| {
            x.expect("Failed to read line")
                .parse::<T>()
                .expect("Failed to parse number")
        })
        .collect()
}

fn count_incrementing<T: PartialOrd>(data: &[T]) -> usize {
    data.iter()
        .zip(&data[1..])
        .filter(|(first, second)| first < second)
        .count()
}

fn part_one() -> usize {
    count_incrementing::<usize>(&read_input("input"))
}

fn part_two() -> usize {
    count_incrementing(
        &read_input("input")
            .windows(3)
            .map(|x| x.iter().sum())
            .collect::<Vec<u32>>(),
    )
}

pub fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
