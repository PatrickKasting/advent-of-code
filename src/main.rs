use std::fmt::Debug;

use anyhow::Context;
use clap::Parser;
use infrastructure::{
    cli::{day, usize_within},
    Day, Puzzle,
};

fn main() -> anyhow::Result<()> {
    let CommandLineArguments { year, day, puzzle } = CommandLineArguments::parse();

    let &answer = [advent21::answer, advent22::answer, advent23::answer]
        .get(year - 2021)
        .context("year should be 2021, 2022, or 2023")?;
    let answer = answer(day, puzzle)?;
    println!("{answer}");

    Ok(())
}

type Year = usize;

#[derive(Debug, Clone, Copy, Parser)]
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

fn year(str: &str) -> anyhow::Result<Year> {
    usize_within(2021..=2023, str)
}
