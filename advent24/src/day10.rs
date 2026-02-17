use shared::grid::Grid;

type TopographicMap = Grid<usize>;

pub fn first_answer(input: &str) -> String {
    let map = TopographicMap::from(input);
    for pos in map.edge_positions_clockwise() {
        println!("{pos:?}");
    }
    todo!()
}

pub fn second_answer(input: &str) -> String {
    let map = TopographicMap::from(input);

    todo!()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 10;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 14);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 396);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 34);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1200);
    }
}
