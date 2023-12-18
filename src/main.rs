mod day1;
mod day2;
mod day3;
mod day4;

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

const SOLUTIONS: [(Solution, Solution); 4] = [
    (day1::first, day1::second),
    (day2::first, day2::second),
    (day3::first, day3::second),
    (day4::first, day4::second),
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
    use super::*;

    pub fn test_on_input(day: Day, puzzle: Puzzle, input: Input, expected: impl ToString) {
        let actual = solution(day, puzzle)(super::input(day, input));
        assert_eq!(actual, expected.to_string());
    }
}
