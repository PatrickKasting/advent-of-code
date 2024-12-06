use easy_cast::Cast;
use itertools::Itertools;
use shared::{
    grid::{self, Direction, Grid, Position},
    vector::{RotationInTwoDimensions, Vector},
};

type WordSearch = Grid<Letter>;
type Letter = char;

pub fn first_answer(input: &str) -> String {
    let word_search = WordSearch::from(input);
    xmas_count(&word_search).to_string()
}

pub fn second_answer(input: &str) -> String {
    let word_search = WordSearch::from(input);
    x_mas_count(&word_search).to_string()
}

fn xmas_count(word_search: &WordSearch) -> usize {
    word_search
        .iter_row_major()
        .map(|(position, _)| position)
        .cartesian_product(grid::DIRECTIONS_INCLUDING_DIAGONAL)
        .filter(|&(position, direction)| is_xmas_match(word_search, position, direction))
        .count()
}

fn is_xmas_match(word_search: &WordSearch, mut position: Position, direction: Direction) -> bool {
    for char in "XMAS".chars() {
        if word_search.get(position) != Some(&char) {
            return false;
        }
        position = position.add(direction);
    }
    true
}

fn x_mas_count(word_search: &WordSearch) -> usize {
    let rows = 1..word_search.height() - 1;
    let columns = 1..word_search.width() - 1;
    rows.cartesian_product(columns)
        .filter(|&(row, column)| is_x_mas_match(word_search, [row, column].cast()))
        .count()
}

fn is_x_mas_match(word_search: &WordSearch, center: Position) -> bool {
    if word_search[center] != 'A' {
        return false;
    }

    let mut directions = [[-1, -1], [-1, 1], [1, 1], [1, -1]];
    for _ in 0..4 {
        let diagonal_neighbors = directions.map(|direction| word_search[center.add(direction)]);
        if diagonal_neighbors == ['M', 'M', 'S', 'S'] {
            return true;
        }
        directions = directions.map(RotationInTwoDimensions::right);
    }
    false
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 4;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 18);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 2344);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 9);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1815);
    }
}
