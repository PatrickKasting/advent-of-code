use std::ops::{Add, Mul};

use shared::{number_theory::number_of_decimal_digits, string::isizes};

type TestValue = Number;
type Number = isize;

pub fn first_answer(input: &str) -> String {
    let operators = [<Number as Add>::add, <Number as Mul>::mul];
    calibration_result(input, operators).to_string()
}

pub fn second_answer(input: &str) -> String {
    let operators = [<Number as Add>::add, <Number as Mul>::mul, concatenation];
    calibration_result(input, operators).to_string()
}

fn calibration_result<const NUM_OPERATORS: usize>(
    input: &str,
    operators: [fn(Number, Number) -> Number; NUM_OPERATORS],
) -> Number {
    equations(input)
        .filter(|(test_value, numbers)| is_solvable(operators, *test_value, 0, numbers))
        .map(|(test_value, _)| test_value)
        .sum::<Number>()
}

fn concatenation(lhs: Number, rhs: Number) -> Number {
    lhs * (10 as Number).pow(number_of_decimal_digits(rhs)) + rhs
}

fn is_solvable<const NUM_OPERATORS: usize>(
    operators: [fn(Number, Number) -> Number; NUM_OPERATORS],
    test_value: TestValue,
    accumulated: Number,
    numbers: &[Number],
) -> bool {
    let [head, tail @ ..] = numbers else {
        return test_value == accumulated;
    };
    operators
        .into_iter()
        .map(|operator| operator(accumulated, *head))
        .any(|accumulated| {
            accumulated <= test_value && is_solvable(operators, test_value, accumulated, tail)
        })
}

fn equations(input: &str) -> impl Iterator<Item = (TestValue, Vec<Number>)> + '_ {
    input.lines().map(equation)
}

fn equation(line: &str) -> (TestValue, Vec<Number>) {
    let (test_value, numbers) = line
        .split_once(": ")
        .expect("test value and numbers should be separated by a colon");
    let test_value = test_value.parse().expect("test value should be numberic");
    (test_value, isizes(numbers))
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};
    use itertools::Itertools;

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 7;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 3749);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(
            DAY,
            Puzzle::First,
            Input::PuzzleInput,
            10_741_443_549_536_usize,
        );
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 11387);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            500_335_179_214_836_usize,
        );
    }

    #[test]
    fn is_solvable() {
        let function = |(test_value, numbers): (Number, Vec<Number>)| {
            super::is_solvable(
                [<Number as Add>::add, <Number as Mul>::mul],
                test_value,
                0,
                numbers.as_slice(),
            )
        };
        let input = input(DAY, Input::Example(0));
        let cases =
            equations(&input).zip_eq([true, true, false, false, false, false, false, false, true]);
        test::cases(function, cases);
    }
}
