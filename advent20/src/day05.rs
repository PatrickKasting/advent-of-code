use easy_cast::Cast;
use itertools::Itertools;

type SeatId = usize;

pub fn first_answer(input: &str) -> String {
    input
        .lines()
        .map(seat_id)
        .max()
        .expect("at least one seat specification should be in the list")
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    input
        .lines()
        .map(seat_id)
        .sorted_unstable()
        .tuple_windows()
        .find(|(some, next)| next - some == 2)
        .map(|(predecessor, _)| predecessor + 1)
        .expect("missing seat id should be between two others")
        .to_string()
}

fn seat_id(line: &str) -> SeatId {
    debug_assert_eq!(line.len(), 10, "seat specification should be 10 characters");
    let row = binary_partitioning(&line[0..7], 'F', 'B');
    let column = binary_partitioning(&line[7..10], 'L', 'R');
    row * 8 + column
}

fn binary_partitioning(line: &str, lower_half: char, upper_half: char) -> usize {
    let mut range = 0..2_usize.pow(line.len().cast());
    for char in line.chars() {
        let half = (range.end - range.start) / 2;
        if char == lower_half {
            range.end -= half;
        } else if char == upper_half {
            range.start += half;
        } else {
            panic!("char should be upper-half token or lower-half token")
        }
    }
    debug_assert_eq!(
        range.end - range.start,
        1,
        "only one possibility should remain"
    );
    range.start
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 5;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 820);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 864);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 739);
    }
}
