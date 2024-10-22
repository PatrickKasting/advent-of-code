mod day01;

const SOLUTIONS: &[[infrastructure::Solution; 2]] = &[];

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
