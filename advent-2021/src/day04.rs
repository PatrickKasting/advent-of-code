const BOARD_SIZE: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Board {
    board: [u8; BOARD_SIZE * BOARD_SIZE],
    marks: [bool; BOARD_SIZE * BOARD_SIZE],
}

pub fn first(input: &str) -> String {
    let (numbers, mut boards) = parse_input(input);
    for number in numbers {
        for board in boards.iter_mut() {
            if board.mark(number) {
                return board.score(number).to_string();
            }
        }
    }
    panic!("bingo should finish before the numbers run out");
}

pub fn second(input: &str) -> String {
    let (numbers, mut boards) = parse_input(input);
    for number in numbers {
        let mut board_index = 0;
        while board_index < boards.len() {
            if boards[board_index].mark(number) {
                let finished_board = boards.swap_remove(board_index);
                if boards.is_empty() {
                    return finished_board.score(number).to_string();
                }
            } else {
                board_index += 1;
            }
        }
    }
    panic!("bingo should end with a single board");
}

impl Board {
    fn from_lines<'lines, 'input>(lines: &'lines mut impl Iterator<Item = &'input str>) -> Self {
        let mut board = [0; BOARD_SIZE * BOARD_SIZE];
        for row in 0..BOARD_SIZE {
            let line = lines
                .next()
                .expect("input should contain only entire boards");
            for (col, number) in line
                .split_whitespace()
                .map(|number| number.parse().expect("boards should contain only numbers"))
                .enumerate()
            {
                board[row * BOARD_SIZE + col] = number;
            }
        }
        let marks = [false; BOARD_SIZE * BOARD_SIZE];
        Board { board, marks }
    }

    fn mark(&mut self, number: u8) -> bool {
        if let Some(pos) = self
            .board
            .iter()
            .position(|&board_number| number == board_number)
        {
            self.marks[pos] = true;
            return self.wins(pos);
        }
        false
    }

    fn wins(&self, mark: usize) -> bool {
        let (row, col) = (mark / BOARD_SIZE, mark % BOARD_SIZE);
        let mut row_positions = row * BOARD_SIZE..(row + 1) * BOARD_SIZE;
        let mut col_positions = (col..BOARD_SIZE * BOARD_SIZE).step_by(BOARD_SIZE);
        row_positions.all(|pos| self.marks[pos]) || col_positions.all(|pos| self.marks[pos])
    }

    fn score(&self, number: u8) -> usize {
        let sum_unmarked: usize = self
            .board
            .iter()
            .zip(self.marks.iter())
            .filter(|(_, &mark)| !mark)
            .map(|(&board_number, _)| board_number as usize)
            .sum();
        sum_unmarked * number as usize
    }
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut lines = input.lines();
    let numbers: Vec<u8> = lines
        .next()
        .expect("input should have a line of numbers")
        .split(',')
        .map(|number| {
            number
                .parse()
                .expect("the first line should be comma-separated numbers")
        })
        .collect();
    let mut boards = Vec::new();
    while lines.next().is_some() {
        boards.push(Board::from_lines(&mut lines))
    }
    (numbers, boards)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 4;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 4512);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 34506);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 1924);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 7686);
    }
}
