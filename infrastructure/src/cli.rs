use std::ops::RangeInclusive;

use anyhow::anyhow;
use clap::Parser;

use crate::{Day, Puzzle};

#[derive(Debug, Clone, Copy, Parser)]
#[command(about, long_about = None)]
struct CommandLineArguments {
    /// Which day?
    #[clap(value_parser=day)]
    day: Day,

    /// First or second puzzle?
    puzzle: Puzzle,
}

/// # Errors
///
/// Errors if the wanted puzzle-input file cannot be read.
pub fn main(answer: fn(Day, Puzzle) -> anyhow::Result<String>) -> anyhow::Result<()> {
    let CommandLineArguments { day, puzzle } = CommandLineArguments::parse();
    let answer = answer(day, puzzle)?;
    println!("{answer}");
    Ok(())
}

/// # Errors
///
/// Errors if the given day is outside `1..=25`.
pub fn day(str: &str) -> anyhow::Result<Day> {
    usize_within(1..=25, str)
}

/// # Errors
///
/// Errors if the given `str` does not parse to a `usize` within the given range.
pub fn usize_within(range: RangeInclusive<usize>, str: &str) -> anyhow::Result<usize> {
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
