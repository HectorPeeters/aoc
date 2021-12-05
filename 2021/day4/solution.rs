use std::{
    fs::File,
    io::{BufReader, *},
};

fn read_input(file: &str) -> (Vec<u32>, Vec<Vec<Vec<u32>>>) {
    let file = File::open(file).expect("No input file");
    let mut lines = BufReader::new(file).lines();

    let draws = lines
        .nth(0)
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut boards: Vec<Vec<Vec<u32>>> = vec![vec![]];

    for line in lines.skip(1) {
        let line = line.unwrap();
        if line.is_empty() {
            boards.push(vec![]);
        }

        let values = line
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        if !values.is_empty() {
            boards.last_mut().unwrap().push(values);
        }
    }

    (draws, boards)
}

fn check_win(board: &[Vec<u32>], draws: &[u32]) -> bool {
    board.iter().any(|x| x.iter().all(|y| draws.contains(y)))
        || (0..board.len()).any(|x| board.iter().all(|y| draws.contains(&y[x])))
}

fn count_score(board: &[Vec<u32>], draws: &[u32]) -> u32 {
    board
        .iter()
        .flatten()
        .filter(|x| !draws.contains(x))
        .sum::<u32>()
        * draws.last().unwrap()
}

fn part_one() -> u32 {
    let (draws, boards) = read_input("input");

    for i in 1..(draws.len() + 1) {
        for board in &boards {
            if check_win(board, &draws[..i]) {
                return count_score(board, &draws[..i]);
            }
        }
    }

    unreachable!()
}

fn part_two() -> u32 {
    let (draws, boards) = read_input("input");

    for i in (1..draws.len()).rev() {
        for board in &boards {
            if !check_win(board, &draws[..i]) {
                return count_score(board, &draws[..i + 1]);
            }
        }
    }

    unreachable!()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
