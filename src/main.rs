#![deny(unsafe_code)]
#![deny(non_ascii_idents)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::empty_enum_variants_with_brackets)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::fn_to_numeric_cast_any)]
#![warn(clippy::format_push_string)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::infinite_loop)]
#![warn(clippy::let_underscore_must_use)]
#![warn(clippy::mem_forget)]
#![warn(clippy::mixed_read_write_in_expression)]
#![warn(clippy::missing_assert_message)]
#![warn(clippy::mod_module_files)]
#![warn(clippy::mutex_atomic)]
#![warn(clippy::needless_raw_strings)]
#![warn(clippy::partial_pub_fields)]
#![warn(clippy::pub_with_shorthand)]
#![warn(clippy::ref_patterns)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::semicolon_inside_block)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_add)]
#![warn(clippy::string_to_string)]
#![warn(clippy::tests_outside_test_module)]
#![warn(clippy::todo)]
#![warn(clippy::try_err)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unnecessary_safety_comment)]
#![warn(clippy::unnecessary_safety_doc)]
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::use_debug)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::wildcard_enum_match_arm)]

mod advent2022;
mod advent2023;
mod data_structures;
mod math;
mod matrix;
mod search;
mod strings;

use std::{fmt::Debug, fs, ops::RangeInclusive};

use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use strum::EnumString;

fn usize_within(range: RangeInclusive<usize>, str: &str) -> Result<usize> {
    let usize = str
        .parse()
        .map_err(|_| anyhow!("value should be a positive number"))?;
    if range.contains(&usize) {
        Ok(usize)
    } else {
        Err(anyhow!(
            "value should be between {} and {}",
            range.start(),
            range.end(),
        ))
    }
}

type Year = usize;

fn year(str: &str) -> Result<Year> {
    usize_within(2022..=2023, str)
}

type Day = usize;

fn day(str: &str) -> Result<Day> {
    usize_within(1..=25, str)
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
    /// Which year?
    #[clap(value_parser=year)]
    year: Year,

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

fn input(year: usize, day: usize, input: Input) -> String {
    let path = match input {
        Input::Example(example) => format!("examples/{year}/{day:02}/{example}.txt"),
        Input::PuzzleInput => format!("puzzle-inputs/{year}/{day:02}.txt"),
    };
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("'{path}' should exist"))
}

type Solution = fn(&str) -> String;

fn solution(year: usize, day: usize, puzzle: Puzzle) -> Solution {
    let solution = match year {
        2022 => advent2022::solution,
        2023 => advent2023::solution,
        _ => panic!("year should be 2022 or 2023"),
    };
    solution(day, puzzle)
}

fn main() {
    let command_line_arguments = CommandLineArguments::parse();

    let input = input(
        command_line_arguments.year,
        command_line_arguments.day,
        Input::PuzzleInput,
    );
    let solution = solution(
        command_line_arguments.year,
        command_line_arguments.day,
        command_line_arguments.puzzle,
    );
    let answer = solution(&input);
    println!("{answer}");
}

#[cfg(test)]
pub mod tests {
    use std::fmt::Display;

    use itertools::Itertools;

    use super::*;

    /// # Panics
    ///
    /// Panics if the return value of the solution applied to the input does not equal
    /// `expected.to_string()`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn test_on_input(
        year: Year,
        day: Day,
        puzzle: Puzzle,
        input: Input,
        expected: impl Display,
    ) {
        let actual = solution(year, day, puzzle)(&super::input(year, day, input));
        assert_eq!(
            actual,
            expected.to_string(),
            "actual answer '{actual}' should equal expected answer '{expected}'"
        );
    }

    /// # Panics
    ///
    /// Panics if there is a mismatch between the return value of `function` applied to a test case
    /// from `cases` and the corresponding expected answer from `expected`. Also panics if the
    /// number of test cases and the number of expected answers differ.
    pub fn test_cases<Case: Debug + Clone, Answer: Debug + Eq>(
        mut function: impl FnMut(Case) -> Answer,
        cases: impl IntoIterator<Item = Case>,
        expected: impl IntoIterator<Item = Answer>,
    ) {
        for (case, expected) in cases.into_iter().zip_eq(expected) {
            let actual = function(case.clone());
            assert_eq!(
                actual, expected,
                "answer to case '{case:?}' should match expected"
            );
        }
    }
}
