use aoc::{
    error::{AocError, Result},
    year2021::*,
    Solution,
};

const FIRST_YEAR: usize = 2021;

macro_rules! solution {
    ($solutions:expr, $year:expr, $day:expr, $sol:ident) => {
        $solutions[$year - FIRST_YEAR][$day] = Some(Box::<$sol>::default());
    };
}

fn main() -> Result<()> {
    let mut solutions: [[Option<Box<dyn Solution>>; 25]; 1] = Default::default();
    solution!(solutions, 2021, 1, Solution2021Day1);
    solution!(solutions, 2021, 2, Solution2021Day2);
    solution!(solutions, 2021, 3, Solution2021Day3);
    solution!(solutions, 2021, 4, Solution2021Day4);
    solution!(solutions, 2021, 5, Solution2021Day5);
    solution!(solutions, 2021, 6, Solution2021Day6);
    solution!(solutions, 2021, 7, Solution2021Day7);

    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        3 => {
            let year: usize = args[1].parse()?;
            let day: usize = args[2].parse()?;

            let solution = &mut solutions[year - FIRST_YEAR][day];

            match solution.as_mut() {
                Some(solution) => {
                    solution.parse()?;

                    println!("Part 1: {}", solution.part1()?);
                    println!("Part 2: {}", solution.part2()?);

                    Ok(())
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
