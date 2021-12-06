use std::cmp::{max, min};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, *},
};

type Point = (isize, isize);

fn read_input(file: &str) -> Vec<(Point, Point)> {
    let file = File::open(file).expect("No input file");
    let lines = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let mut result = vec![];

    for line in &lines {
        let coords = line.split(" -> ").collect::<Vec<_>>();

        let start = coords[0]
            .split(",")
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        let end = coords[1]
            .split(",")
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        result.push(((start[0], start[1]), (end[0], end[1])));
    }

    result
}

fn part_one() -> usize {
    let points = read_input("input");

    let points = points
        .into_iter()
        .map(|(start, end)| {
            let mut line_points = vec![];

            if start.0 == end.0 {
                for y in min(start.1, end.1)..max(start.1, end.1) + 1 {
                    line_points.push((start.0, y));
                }
            } else if start.1 == end.1 {
                for x in min(start.0, end.0)..max(start.0, end.0) + 1 {
                    line_points.push((x, start.1));
                }
            }
            line_points
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut map: HashMap<Point, isize> = HashMap::new();

    for p in &points {
        if map.contains_key(p) {
            map.insert(*p, map.get(p).unwrap() + 1);
        } else {
            map.insert(*p, 1);
        }
    }

    map.into_iter().filter(|(_, v)| *v > 1).count()
}

fn part_two() -> usize {
    let points = read_input("input");

    let points = points
        .into_iter()
        .map(|(start, end)| {
            let mut line_points = vec![];
            let dx = end.0 - start.0;
            let dy = end.1 - start.1;

            if dx == 0 {
                for y in min(start.1, end.1)..max(start.1, end.1) + 1 {
                    line_points.push((start.0, y));
                }
            } else if dy == 0 {
                for x in min(start.0, end.0)..max(start.0, end.0) + 1 {
                    line_points.push((x, start.1));
                }
            } else {
                assert_eq!(dx.abs(), dy.abs());
                for i in 0..dx.abs() + 1 {
                    line_points.push((start.0 + dx.signum() * i, start.1 + dy.signum() * i));
                }
            }
            line_points
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut map: HashMap<Point, isize> = HashMap::new();

    for p in &points {
        if map.contains_key(p) {
            map.insert(*p, map.get(p).unwrap() + 1);
        } else {
            map.insert(*p, 1);
        }
    }

    map.into_iter().filter(|(_, v)| *v > 1).count()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
