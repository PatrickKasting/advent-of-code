use itertools::Itertools;

type Number = usize;

pub fn first_answer(input: &str) -> String {
    product_of_numbers_that_sums_to(input, 2020, 2).to_string()
}

pub fn second_answer(input: &str) -> String {
    product_of_numbers_that_sums_to(input, 2020, 3).to_string()
}

fn product_of_numbers_that_sums_to(input: &str, sum: Number, number_of_numbers: usize) -> Number {
    let combination = input
        .lines()
        .map(|line| line.parse::<usize>().expect("line should contain integer"))
        .combinations(number_of_numbers)
        .find(|combination| combination.iter().sum::<Number>() == sum)
        .expect("a single pair should add to 2020");
    combination.into_iter().product()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 1;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 514_579);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 445_536);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 241_861_950);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 138_688_160);
    }
}
