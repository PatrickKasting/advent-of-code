use std::fmt::{Debug, Display};

use crate::{Day, Input, Puzzle, Solution};

/// # Panics
///
/// Panics if the return value of the solution applied to the input does not equal
/// `expected.to_string()`.
pub fn on_input(
    inputs: fn(Day, Input) -> String,
    solutions: &[[Solution; 2]],
    day: Day,
    puzzle: Puzzle,
    input: Input,
    expected: impl Display,
) {
    let actual = solutions[day - 1][puzzle](&inputs(day, input));
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
pub fn cases<Input: Debug + Clone, Answer: Debug + Eq>(
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

/// # Panics
///
/// Panics with the message `msg` followed by `left` and `right`.
pub fn panic_left_right(msg: &str, left: impl Debug, right: impl Debug) {
    let lines = [
        format!("{msg}:"),
        format!("  left: {left:?}"),
        format!(" right: {right:?}"),
    ];
    panic!("{}", lines.join("\n"))
}
