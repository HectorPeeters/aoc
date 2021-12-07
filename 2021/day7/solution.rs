use std::{
    fs::File,
    io::{BufReader, *},
};

fn read_input(file: &str) -> Vec<usize> {
    let file = File::open(file).expect("No input file");
    BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .split(",")
                .map(|y| y.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn part_one() -> usize {
    let crabs = read_input("input");

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    (*min..(max + 1))
        .into_iter()
        .map(|pos| {
            crabs
                .iter()
                .map(|x| (*x as isize - pos as isize).abs())
                .sum::<isize>()
        })
        .min()
        .unwrap() as usize
}

fn part_two() -> usize {
    let crabs = read_input("input");

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    (*min..(max + 1))
        .into_iter()
        .map(|pos| {
            crabs
                .iter()
                .map(|x| {
                    let n = (*x as isize - pos as isize).abs() + 1;
                    n * (n - 1) / 2
                })
                .sum::<isize>()
        })
        .min()
        .unwrap() as usize
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
