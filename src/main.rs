mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod grid;
mod search;
mod utilities;

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
    PuzzleInput,
}

fn input(day: usize, input: Input) -> String {
    let path = match input {
        Input::Example(example) => format!("examples/{day:02}/{example}.txt"),
        Input::PuzzleInput => format!("puzzle-inputs/{day:02}.txt"),
    };
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("'{path}' should exist"))
}

type Solution = fn(String) -> String;

const SOLUTIONS: &[(Solution, Solution)] = &[
    (day01::first, day01::second),
    (day02::first, day02::second),
    (day03::first, day03::second),
    (day04::first, day04::second),
    (day05::first, day05::second),
    (day06::first, day06::second),
    (day07::first, day07::second),
    (day08::first, day08::second),
    (day09::first, day09::second),
    (day10::first, day10::second),
    (day11::first, day11::second),
    (day12::first, day12::second),
    (day13::first, day13::second),
    (day14::first, day14::second),
    (day15::first, day15::second),
    (day16::first, day16::second),
    (day17::first, day17::second),
    (day18::first, day18::second),
    (day19::first, day19::second),
];

fn solution(day: Day, puzzle: Puzzle) -> Solution {
    match puzzle {
        Puzzle::First => SOLUTIONS[day - 1].0,
        Puzzle::Second => SOLUTIONS[day - 1].1,
    }
}

fn main() {
    let command_line_arguments = CommandLineArguments::parse();

    let input = input(command_line_arguments.day, Input::PuzzleInput);
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
