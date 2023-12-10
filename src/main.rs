mod day1;
mod day2;
mod day3;

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
enum Puzzle {
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

fn input(day: usize) -> String {
    fs::read_to_string(format!("input/{day}.txt"))
        .unwrap_or_else(|_| panic!("input for day {day} should exist"))
}

type Solution = fn(String) -> String;

const SOLUTIONS: [(Solution, Solution); 3] = [
    (day1::first, day1::second),
    (day2::first, day2::second),
    (day3::first, day3::second),
];

fn main() {
    let command_line_arguments = CommandLineArguments::parse();

    let input = input(command_line_arguments.day);
    let solution = match command_line_arguments.puzzle {
        Puzzle::First => SOLUTIONS[command_line_arguments.day - 1].0,
        Puzzle::Second => SOLUTIONS[command_line_arguments.day - 1].1,
    };
    let answer = solution(input);
    println!("{}", answer);
}

fn example(day: usize, example: usize) -> String {
    fs::read_to_string(format!("examples/{day}/{example}.txt")).expect("example should exist")
}
