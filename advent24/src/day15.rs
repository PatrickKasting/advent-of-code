use easy_cast::Conv;
use shared::grid::{self, Direction, Grid, Position};

type Warehouse = Grid<u8>;

pub fn first_answer(input: &str) -> String {
    todo!()
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn gps_coordinate([row, column]: Position) -> usize {
    100 * usize::conv(row) + usize::conv(column)
}

fn warehouse_and_movements(input: &str) -> (Warehouse, impl Iterator<Item = Direction> + use<'_>) {
    let (warehouse, movements) = input
        .split_once("\n\n")
        .expect("warehouse and movements should be separated by an empty line");
    let warehouse = Warehouse::from(warehouse);
    let movements = movements.chars().filter_map(grid::direction).cycle();
    (warehouse, movements)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 15;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 10092);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 2028);
    }

    // #[test]
    // fn first_answer_input() {
    //     test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 396);
    // }

    // #[test]
    // fn second_answer_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 34);
    // }

    // #[test]
    // fn second_answer_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1200);
    // }
}
