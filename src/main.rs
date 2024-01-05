mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
pub mod grid;
pub mod math;
pub mod utilities;

use std::fs;

use anyhow::{anyhow, Result};
use clap::Parser;
use strum::EnumString;

type Day = usize;

fn day(str: &str) -> Result<Day> {
    let day = str.parse()?;

    let (min, max) = (1, SOLUTIONS.len());
    if day > max {
        Err(anyhow!("exceeds maximum of {}", max))
    } else if day < min {
        Err(anyhow!("subceeds minimum of {}", min))
    } else {
        Ok(day)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumString)]
pub enum Puzzle {
    #[strum(ascii_case_insensitive)]
    First,

    #[strum(ascii_case_insensitive)]
    Second,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Parser)]
#[command(about, long_about = None)]
struct CommandLineArguments {
    /// Which day?
    #[clap(value_parser=day)]
    day: Day,

    /// First or second puzzle?
    puzzle: Puzzle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Input {
    Example(usize),
    Real,
}

fn input(day: usize, input: Input) -> String {
    let path = match input {
        Input::Example(example) => format!("examples/{day}/{example}.txt"),
        Input::Real => format!("input/{day}.txt"),
    };
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("'{path}' should exist"))
}

type Solution = fn(String) -> String;

const SOLUTIONS: &[(Solution, Solution)] = &[
    (day1::first, day1::second),
    (day2::first, day2::second),
    (day3::first, day3::second),
    (day4::first, day4::second),
    (day5::first, day5::second),
    (day6::first, day6::second),
    (day7::first, day7::second),
    (day8::first, day8::second),
    (day9::first, day9::second),
    (day10::first, day10::second),
    (day11::first, day11::second),
    (day12::first, day12::second),
    (day13::first, day13::second),
];

fn solution(day: Day, puzzle: Puzzle) -> Solution {
    match puzzle {
        Puzzle::First => SOLUTIONS[day - 1].0,
        Puzzle::Second => SOLUTIONS[day - 1].1,
    }
}

fn main() {
    let command_line_arguments = CommandLineArguments::parse();

    let input = input(command_line_arguments.day, Input::Real);
    let solution = solution(command_line_arguments.day, command_line_arguments.puzzle);
    let answer = solution(input);
    println!("{}", answer);
}

#[cfg(test)]
pub mod tests {
    use std::fmt::Debug;

    use itertools::Itertools;

    use super::*;

    pub fn test_on_input(day: Day, puzzle: Puzzle, input: Input, expected: impl ToString) {
        let actual = solution(day, puzzle)(super::input(day, input));
        assert_eq!(actual, expected.to_string());
    }

    pub fn test_cases<Case, Answer: Debug + Eq>(
        function: impl FnMut(Case) -> Answer,
        cases: impl IntoIterator<Item = Case>,
        expected: impl IntoIterator<Item = Answer>,
    ) {
        for (answer, expected) in cases.into_iter().map(function).zip_eq(expected) {
            assert_eq!(answer, expected);
        }
    }
}
