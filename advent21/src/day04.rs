use std::convert::identity;

use itertools::Itertools;
use shared::{grid::Grid, string::usizes};

const BOARD_SIZE: usize = 5;

type Board = (Grid<Number>, Grid<bool>);
type Number = usize;

pub fn first_answer(input: &str) -> String {
    let (numbers, mut boards) = numbers_and_boards(input);
    winning_board_score(numbers, &mut boards).to_string()
}

pub fn second_answer(input: &str) -> String {
    let (numbers, mut boards) = numbers_and_boards(input);
    losing_board_score(numbers, &mut boards).to_string()
}

pub fn winning_board_score(numbers: impl Iterator<Item = Number>, boards: &mut [Board]) -> Number {
    for number in numbers {
        mark(number, boards);
        if let Some(winning_board) = boards.iter().find(|board| has_bingo(board)) {
            return score(winning_board, number);
        }
    }
    panic!("a bingo should occur before the numbers run out");
}

pub fn losing_board_score(
    numbers: impl Iterator<Item = Number>,
    boards: &mut Vec<Board>,
) -> Number {
    for number in numbers {
        mark(number, boards.as_mut_slice());
        if boards.len() == 1 && has_bingo(&boards[0]) {
            return score(&boards[0], number);
        }
        boards.retain(|board| !has_bingo(board));
    }
    panic!("all bingos should occur before the numbers run out");
}

fn mark(number: Number, boards: &mut [Board]) {
    for (numbers, marks) in boards {
        if let Some((position, _)) = numbers.find(|_, &board_number| board_number == number) {
            marks[position] = true;
        }
    }
}

fn has_bingo((_, marks): &Board) -> bool {
    let is_bingo_row = marks.rows().any(|row| row.copied().all(identity));
    let is_bingo_column = marks.columns().any(|column| column.copied().all(identity));
    is_bingo_row || is_bingo_column
}

fn score(board: &Board, last_number: usize) -> usize {
    sum_of_unmarked_numbers(board) * last_number
}

fn sum_of_unmarked_numbers((numbers, marks): &Board) -> Number {
    let all_numbers: Number = numbers.iter_row_major().map(|(_, number)| number).sum();
    let marked_numbers: Number = marks
        .iter_row_major()
        .filter_map(|(position, is_marked)| is_marked.then_some(numbers[position]))
        .sum();
    all_numbers - marked_numbers
}

fn numbers_and_boards(input: &str) -> (impl Iterator<Item = Number> + '_, Vec<Board>) {
    let (numbers, boards) = input
        .split_once("\n\n")
        .expect("numbers and boards should be separated by an empty line");
    (self::numbers(numbers), self::boards(boards))
}

fn numbers(line: &str) -> impl Iterator<Item = Number> + '_ {
    line.split(',')
        .map(str::parse)
        .map(|number| number.expect("number should be numerical"))
}

fn boards(str: &str) -> Vec<Board> {
    str.split("\n\n").map(board).collect_vec()
}

fn board(str: &str) -> Board {
    let numbers = Grid::from_elements(usizes(str), BOARD_SIZE);
    let marks = Grid::new(BOARD_SIZE, BOARD_SIZE, |_| false);
    (numbers, marks)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 4;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 4512);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 34506);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 1924);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 7686);
    }
}
