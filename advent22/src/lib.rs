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
    [day01::first_answer, day01::second_answer],
    [day02::first_answer, day02::second_answer],
    [day03::first_answer, day03::second_answer],
    [day04::first_answer, day04::second_answer],
    [day05::first_answer, day05::second_answer],
    [day06::first_answer, day06::second_answer],
    [day07::first_answer, day07::second_answer],
    [day08::first_answer, day08::second_answer],
    [day09::first_answer, day09::second_answer],
    [day10::first_answer, day10::second_answer],
    [day11::first_answer, day11::second_answer],
    [day12::first_answer, day12::second_answer],
    [day13::first_answer, day13::second_answer],
    [day14::first_answer, day14::second_answer],
    [day15::first_answer, day15::second_answer],
    [day16::first_answer, day16::second_answer],
    [day17::first_answer, day17::second_answer],
    [day18::first_answer, day18::second_answer],
    [day19::first_answer, day19::second_answer],
    [day20::first_answer, day20::second_answer],
    [day21::first_answer, day21::second_answer],
    [day22::first_answer, day22::second_answer],
    [day23::first_answer, day23::second_answer],
    [day24::first_answer, day24::second_answer],
    [day25::first_answer, day25::second_answer],
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
    pub fn test_on_input(day: Day, puzzle: Puzzle, input: Input, expected: impl Display) {
        infrastructure::test::on_input(self::input, super::SOLUTIONS, day, puzzle, input, expected);
    }

    pub fn input(day: infrastructure::Day, input: infrastructure::Input) -> String {
        infrastructure::input(env!("CARGO_MANIFEST_DIR"), day, input).expect("input should exist")
    }
}
