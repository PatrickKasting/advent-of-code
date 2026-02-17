use itertools::Itertools;
use shared::{number_theory::number_of_decimal_digits, string::isizes};

type Line = Vec<Stone>;
type Stone = isize;

pub fn first_answer(input: &str) -> String {
    let mut line = isizes(input);
    for _ in 0..25 {
        line = next_line(line);
    }
    line.len().to_string()
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn next_line(line: Line) -> Line {
    line.into_iter().flat_map(next_stone).collect_vec()
}

fn next_stone(stone: Stone) -> Vec<Stone> {
    let number_of_digits = number_of_decimal_digits(stone);
    if stone == 0 {
        vec![1]
    } else if number_of_digits % 2 == 0 {
        let half_number_of_digits = number_of_digits / 2;
        let divisor = 10_isize.pow(half_number_of_digits);
        vec![stone / divisor, stone % divisor]
    } else {
        vec![stone * 2024]
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 11;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 55312);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 216042);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 34);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1200);
    }

    #[test]
    fn next_stone() {
        let cases = [(0, vec![1]), (2973, vec![29, 73]), (973, vec![1_969_352])];
        test::cases(super::next_stone, cases);
    }
}
