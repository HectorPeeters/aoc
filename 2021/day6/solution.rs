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
    let mut fishes = read_input("input");

    for _day in 0..80 {
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

    fishes.len()
}

fn part_two() -> usize {
    let mut fishes = read_input("input");

    let mut fish_counters = vec![0; 9];

    for fish in &fishes {
        fish_counters[*fish] += 1;
    }

    for _day in 0..256 {
        let dying = fish_counters.remove(0);
        fish_counters.insert(8, dying);
        fish_counters[6] += dying;
    }

    fish_counters.iter().sum()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
