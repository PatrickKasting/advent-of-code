pub fn first_answer(input: &str) -> String {
    todo!()
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 20;

    #[test]
    fn first_answer_example() {
        test_on_input(
            DAY,
            Puzzle::First,
            Input::Example(0),
            20_899_048_083_289_usize,
        );
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 124);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 12);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 228);
    }
}