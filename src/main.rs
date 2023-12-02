use std::time::Instant;

use aoc::{
    error::{AocError, Result},
    year2020::*,
    year2021::*,
    year2022::*,
    year2023::*,
    Solution, SolutionId,
};

const FIRST_YEAR: usize = 2020;
const YEAR_COUNT: usize = 4;

macro_rules! solution {
    ($solutions:expr, $id:expr, $sol:ident) => {{
        let id = SolutionId::from($id);
        $solutions[id.year - FIRST_YEAR][id.day] = Some((id, Box::<$sol>::default()));
    }};
}

fn run_solution(solution: &mut dyn Solution) -> Result<()> {
    let parse_start = Instant::now();
    solution.parse()?;
    let parse_duration = parse_start.elapsed();

    let part1_start = Instant::now();
    println!("Solution part 1: {}", solution.part1()?);
    let part1_duration = part1_start.elapsed();

    let part2_start = Instant::now();
    println!("Solution part 2: {}", solution.part2()?);
    let part2_duration = part2_start.elapsed();

    println!("\nParsing took\t{parse_duration:?}");
    println!("Part 1 took\t{part1_duration:?}");
    println!("part 2 took\t{part2_duration:?}");

    Ok(())
}

type YearSolutions = [Option<(SolutionId, Box<dyn Solution>)>; 25];

fn main() -> Result<()> {
    let mut solutions: [YearSolutions; YEAR_COUNT] = Default::default();
    solution!(solutions, (2020, 1), Solution2020Day1);
    solution!(solutions, (2020, 2), Solution2020Day2);
    solution!(solutions, (2020, 3), Solution2020Day3);
    solution!(solutions, (2020, 4), Solution2020Day4);
    solution!(solutions, (2020, 5), Solution2020Day5);
    solution!(solutions, (2020, 6), Solution2020Day6);
    solution!(solutions, (2020, 6), Solution2020Day6);

    solution!(solutions, (2021, 1), Solution2021Day1);
    solution!(solutions, (2021, 2), Solution2021Day2);
    solution!(solutions, (2021, 3), Solution2021Day3);
    solution!(solutions, (2021, 4), Solution2021Day4);
    solution!(solutions, (2021, 5), Solution2021Day5);
    solution!(solutions, (2021, 6), Solution2021Day6);
    solution!(solutions, (2021, 7), Solution2021Day7);

    solution!(solutions, (2022, 1), Solution2022Day1);

    solution!(solutions, (2023, 1), Solution2023Day1);
    solution!(solutions, (2023, 2), Solution2023Day2);

    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        1 => {
            for year in solutions {
                for (id, mut solution) in year.into_iter().flatten() {
                    println!("\n\n=====================");
                    println!("Running day {} of {}\n", id.day, id.year);

                    run_solution(solution.as_mut())?;
                }
            }
            Ok(())
        }
        3 => {
            let year: usize = args[1].parse()?;
            let day: usize = args[2].parse()?;

            let solution = &mut solutions[year - FIRST_YEAR][day];

            match solution.as_mut() {
                Some((id, solution)) => {
                    println!("Running day {} of {}\n", id.day, id.year);
                    run_solution(solution.as_mut())
                }
                None => Err(AocError::User(format!(
                    "AOC solution for year {} day {} has not been solved yet",
                    year, day
                ))),
            }
        }
        _ => Err(AocError::User(
            "Incorrect arg count. Usage: ./aoc [year] [day]".to_string(),
        )),
    }
}
