use std::fmt::Debug;

use anyhow::Context;
use clap::Parser;
use infrastructure::{
    cli::{day, usize_within},
    Day, Puzzle,
};

fn main() -> anyhow::Result<()> {
    let CommandLineArguments { year, day, puzzle } = CommandLineArguments::parse();

    let &answer = [
        advent20::answer,
        advent21::answer,
        advent22::answer,
        advent23::answer,
        advent24::answer,
    ]
    .get(year - 2020)
    .context("year should be 2020, 2021, 2022, 2023, or 2024")?;
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
    usize_within(2020..=2024, str)
}
