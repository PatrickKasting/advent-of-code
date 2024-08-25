pub mod cli;
pub mod test;

use std::{fmt::Debug, fs, ops::Index};

use anyhow::{anyhow, Context};
use strum::EnumString;

pub type Day = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum Puzzle {
    #[strum(ascii_case_insensitive)]
    First,

    #[strum(ascii_case_insensitive)]
    Second,
}

impl<T> Index<Puzzle> for [T] {
    type Output = T;

    fn index(&self, puzzle: Puzzle) -> &Self::Output {
        match puzzle {
            Puzzle::First => &self[0],
            Puzzle::Second => &self[1],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Example(usize),
    PuzzleInput,
}

/// # Errors
///
/// Errors if the wanted puzzle-input file cannot be read.
pub fn input(manifest_directory: &str, day: Day, input: Input) -> anyhow::Result<String> {
    let path = match input {
        Input::Example(example) => format!("{manifest_directory}/examples/{day:02}/{example}.txt"),
        Input::PuzzleInput => format!("{manifest_directory}/puzzle-inputs/{day:02}.txt"),
    };
    fs::read_to_string(&path).map_err(|_| anyhow!("path '{path}' should exist"))
}

pub type Solution = fn(&str) -> Answer;
pub type Answer = String;

/// # Errors
///
/// Errors if the wanted puzzle input cannot be read or the wanted solution does not exist.
pub fn answer(
    manifest_directory: &str,
    day: Day,
    puzzle: Puzzle,
    solutions: &[[Solution; 2]],
) -> anyhow::Result<Answer> {
    let input = input(manifest_directory, day, Input::PuzzleInput)?;
    let solution = solutions
        .get(day - 1)
        .context("solution to day should exist")?[puzzle];
    Ok(solution(&input))
}
