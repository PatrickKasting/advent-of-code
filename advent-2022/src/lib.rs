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
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

const SOLUTIONS: &[[infrastructure::Solution; 2]] = &[
    [day01::first, day01::second],
    [day02::first, day02::second],
    [day03::first, day03::second],
    [day04::first, day04::second],
    [day05::first, day05::second],
    [day06::first, day06::second],
    [day07::first, day07::second],
    [day08::first, day08::second],
    [day09::first, day09::second],
    [day10::first, day10::second],
    [day11::first, day11::second],
    [day12::first, day12::second],
    [day13::first, day13::second],
    [day14::first, day14::second],
    [day15::first, day15::second],
    [day16::first, day16::second],
    [day17::first, day17::second],
    [day18::first, day18::second],
    [day19::first, day19::second],
    [day20::first, day20::second],
    [day21::first, day21::second],
    [day22::first, day22::second],
    [day23::first, day23::second],
    [day24::first, day24::second],
    [day25::first, day25::second],
];

/// # Errors
///
/// Errors if the wanted puzzle input cannot be read or the wanted solution does not exist.
pub fn answer(day: infrastructure::Day, puzzle: infrastructure::Puzzle) -> anyhow::Result<String> {
    infrastructure::answer(env!("CARGO_MANIFEST_DIR"), day, puzzle, SOLUTIONS)
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use infrastructure::{Day, Input, Puzzle};

    /// # Panics
    ///
    /// Panics if the return value of the solution applied to the input does not equal
    /// `expected.to_string()`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn test_on_input(day: Day, puzzle: Puzzle, input: Input, expected: impl Display) {
        infrastructure::test::on_input(self::input, super::SOLUTIONS, day, puzzle, input, expected);
    }

    pub fn input(day: infrastructure::Day, input: infrastructure::Input) -> String {
        infrastructure::input(env!("CARGO_MANIFEST_DIR"), day, input).expect("input should exist")
    }
}
