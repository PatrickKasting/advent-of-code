use ahash::AHashMap;
use shared::string::usizes;

type Number = usize;
type Time = usize;

pub fn first_answer(input: &str) -> String {
    let starting_numbers = usizes(input);
    spoken_number(&starting_numbers, 2020).to_string()
}

pub fn second_answer(input: &str) -> String {
    let starting_numbers = usizes(input);
    spoken_number(&starting_numbers, 30_000_000).to_string()
}

fn spoken_number(starting_numbers: &[Number], index: usize) -> Number {
    let mut spoken_numbers: AHashMap<Number, Time> = starting_numbers
        .iter()
        .take(starting_numbers.len() - 1)
        .copied()
        .enumerate()
        .map(|(turn, number)| (number, turn))
        .collect();
    let mut previous_number = *starting_numbers
        .last()
        .expect("at least one starting number should be given");
    for this_turn in starting_numbers.len()..index {
        let last_turn = this_turn - 1;
        let next_number =
            if let Some(last_occurence) = spoken_numbers.insert(previous_number, last_turn) {
                last_turn - last_occurence
            } else {
                0
            };
        previous_number = next_number;
    }
    previous_number
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 15;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 436);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 614);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 175_594);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1065);
    }
}
