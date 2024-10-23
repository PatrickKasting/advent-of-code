use easy_cast::Conv;
use shared::{
    grid::{Coordinate, Direction, Grid},
    vector::Vector,
};

type Map = Grid<char>;

pub fn first_answer(input: &str) -> String {
    let map = Map::from(input);
    number_of_trees_encountered(&map, [1, 3]).to_string()
}

pub fn second_answer(input: &str) -> String {
    let map = Map::from(input);
    let slopes = [[1, 1], [1, 3], [1, 5], [1, 7], [2, 1]];
    slopes
        .into_iter()
        .map(|slope| number_of_trees_encountered(&map, slope))
        .product::<usize>()
        .to_string()
}

fn number_of_trees_encountered(map: &Map, slope: Direction) -> usize {
    let positions = itertools::iterate([0, 0], |&position| position.add(slope))
        .take_while(|&[row, _]| row < Coordinate::conv(map.height()));
    positions
        .filter(|&[row, column]| map[[row, column % Coordinate::conv(map.width())]] == '#')
        .count()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 3;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 7);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 280);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 336);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 4_355_551_200_usize);
    }
}
