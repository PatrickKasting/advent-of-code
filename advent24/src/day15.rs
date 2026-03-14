use easy_cast::Conv;
use shared::{
    grid::{self, Direction, Grid, Position},
    vector::Vector,
};

type Warehouse = Grid<u8>;
type GpsCoordinate = usize;

pub fn first_answer(input: &str) -> String {
    let (mut warehouse, directions) = warehouse_and_directions(input);
    sum_of_gps_coordinates(&mut warehouse, directions).to_string()
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn sum_of_gps_coordinates(
    warehouse: &mut Warehouse,
    directions: impl Iterator<Item = Direction>,
) -> GpsCoordinate {
    state_after(warehouse, directions);
    warehouse
        .iter_row_major()
        .filter(|&(_, &element)| element == b'O')
        .map(|(position, _)| gps_coordinate(position))
        .sum()
}

fn state_after(warehouse: &mut Warehouse, directions: impl Iterator<Item = Direction>) {
    for direction in directions {
        movement(warehouse, direction);
    }
}

fn movement(warehouse: &mut Warehouse, direction: Direction) {
    let (robot, _) = warehouse
        .find(|_, &element| element == b'@')
        .expect("robot should be present");
    let mut current = robot;
    loop {
        current = current.add(direction);
        match warehouse[current] {
            b'O' => (),
            b'#' => return,
            b'.' => {
                warehouse[current] = b'O';
                warehouse[robot.add(direction)] = b'@';
                warehouse[robot] = b'.';
                return;
            }
            _ => panic!("warehouse should contain only expected elements"),
        }
    }
}

fn gps_coordinate([row, column]: Position) -> GpsCoordinate {
    100 * usize::conv(row) + usize::conv(column)
}

fn warehouse_and_directions(input: &str) -> (Warehouse, impl Iterator<Item = Direction> + use<'_>) {
    let (warehouse, directions) = input
        .split_once("\n\n")
        .expect("warehouse and directions should be separated by an empty line");
    let warehouse = Warehouse::from(warehouse);
    let directions = directions.chars().filter_map(grid::direction);
    (warehouse, directions)
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

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 396);
    }

    // #[test]
    // fn second_answer_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 34);
    // }

    // #[test]
    // fn second_answer_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1200);
    // }
}
