use itertools::Itertools;

type Number = usize;

pub fn first_answer(input: &str) -> String {
    let numbers = numbers(input);
    first_number_not_sum_of_two_previous(&numbers, 25).to_string()
}

pub fn second_answer(input: &str) -> String {
    let numbers = numbers(input);
    encryption_weakness(&numbers, 25).to_string()
}

fn first_number_not_sum_of_two_previous(numbers: &[Number], number_of_previous: usize) -> Number {
    numbers
        .windows(number_of_previous + 1)
        .find_map(|window| {
            let current = window[number_of_previous];
            let previous = &window[0..number_of_previous];
            (!is_sum_of_two(previous, current)).then_some(current)
        })
        .expect("at least one number should not be the sum of a pair of preceding numbers")
}

fn is_sum_of_two(terms: &[Number], sum: Number) -> bool {
    terms
        .iter()
        .combinations(2)
        .any(|pair| pair[0] + pair[1] == sum)
}

fn encryption_weakness(numbers: &[Number], number_of_previous: usize) -> Number {
    let invalid_number = first_number_not_sum_of_two_previous(numbers, number_of_previous);
    let contiguous_set = contiguous_set(numbers, invalid_number);
    let (min, max) = contiguous_set
        .iter()
        .copied()
        .minmax()
        .into_option()
        .expect("contiguous set should not be empty");
    min + max
}

fn contiguous_set(numbers: &[Number], sum: Number) -> &[Number] {
    for size in 2..numbers.len() {
        for set in numbers.windows(size) {
            if set.iter().sum::<Number>() == sum {
                return set;
            }
        }
    }
    unreachable!("at least one contiguos set should have the given sum")
}

fn numbers(input: &str) -> Vec<Number> {
    input
        .lines()
        .map(|line| line.parse().expect("line should contain number"))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 9;

    #[test]
    fn first_answer_example() {
        let numbers = numbers(&input(DAY, Input::Example(0)));
        let actual = first_number_not_sum_of_two_previous(&numbers, 5);
        let expected = 127;
        assert_eq!(actual, expected);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 31_161_678);
    }

    #[test]
    fn second_answer_example() {
        let numbers = numbers(&input(DAY, Input::Example(0)));
        let actual = encryption_weakness(&numbers, 5);
        let expected = 62;
        assert_eq!(actual, expected);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 5_453_868);
    }

    #[test]
    fn is_sum_of_two() {
        let function = |(terms, sum)| super::is_sum_of_two(terms, sum);
        let cases: [((&[Number], Number), bool); 3] = [
            ((&[35, 20, 15, 25, 47], 40), true),
            ((&[40, 62, 55, 65, 95], 102), true),
            ((&[95, 102, 117, 150, 182], 127), false),
        ];
        test::cases(function, cases);
    }
}
