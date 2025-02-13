use ahash::AHashSet;

pub fn first_answer(input: &str) -> String {
    index_of_marker(4, input.as_bytes()).to_string()
}

pub fn second_answer(input: &str) -> String {
    index_of_marker(14, input.as_bytes()).to_string()
}

fn index_of_marker(marker_size: usize, datastream: &[u8]) -> usize {
    let mut bytes: AHashSet<u8> = AHashSet::with_capacity(marker_size);
    datastream
        .windows(marker_size)
        .position(|window| {
            bytes.clear();
            bytes.extend(window);
            bytes.len() == marker_size
        })
        .expect("start of package marker should be in datastream")
        + marker_size
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 6;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 7);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 5);
        test_on_input(DAY, Puzzle::First, Input::Example(2), 6);
        test_on_input(DAY, Puzzle::First, Input::Example(3), 10);
        test_on_input(DAY, Puzzle::First, Input::Example(4), 11);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1582);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 19);
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 23);
        test_on_input(DAY, Puzzle::Second, Input::Example(2), 23);
        test_on_input(DAY, Puzzle::Second, Input::Example(3), 29);
        test_on_input(DAY, Puzzle::Second, Input::Example(4), 26);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 3588);
    }
}
