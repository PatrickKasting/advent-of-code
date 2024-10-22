use itertools::Itertools;

type Measurement = usize;

pub fn first_answer(input: &str) -> String {
    number_of_increasing(measurements(input)).to_string()
}

pub fn second_answer(input: &str) -> String {
    let window_sums = measurements(input)
        .tuple_windows()
        .map(|(first, second, third)| first + second + third);
    number_of_increasing(window_sums).to_string()
}

fn number_of_increasing<T: Copy + Ord>(sequence: impl Iterator<Item = T>) -> usize {
    sequence
        .tuple_windows()
        .filter(|&(first, second)| first < second)
        .count()
}

fn measurements(input: &str) -> impl Iterator<Item = Measurement> + '_ {
    input.lines().map(|line| {
        line.parse()
            .expect("every measurement should be a positive integer")
    })
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 1;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 7);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1233);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 5);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1275);
    }
}
