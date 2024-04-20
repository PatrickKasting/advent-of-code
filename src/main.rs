#![deny(unsafe_code, non_ascii_idents)]
#![warn(clippy::all, clippy::pedantic)]
#![warn(
    clippy::allow_attributes_without_reason,
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::empty_enum_variants_with_brackets,
    clippy::empty_structs_with_brackets,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::if_then_some_else_none,
    clippy::infinite_loop,
    clippy::let_underscore_must_use,
    clippy::mem_forget,
    clippy::mixed_read_write_in_expression,
    clippy::missing_assert_message,
    clippy::mod_module_files,
    clippy::mutex_atomic,
    clippy::needless_raw_strings,
    clippy::partial_pub_fields,
    clippy::pub_with_shorthand,
    clippy::ref_patterns,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::semicolon_inside_block,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_to_string,
    clippy::tests_outside_test_module,
    clippy::todo,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::use_debug,
    clippy::unwrap_used,
    clippy::wildcard_enum_match_arm
)]

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
    /// Panics if there is a mismatch between the return value of `function` and the expected
    /// answer.
    pub fn test_cases<Input: Debug + Clone, Answer: Debug + Eq>(
        mut function: impl FnMut(Input) -> Answer,
        cases: impl IntoIterator<Item = (Input, Answer)>,
    ) {
        for (case, expected) in cases {
            let actual = function(case.clone());
            assert_eq!(
                actual, expected,
                "answer to case '{case:?}' should match expected"
            );
        }
    }
}
