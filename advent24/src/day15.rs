use easy_cast::Conv;
use itertools::Itertools;
use shared::{
    grid::{self, Direction, Grid, Position, EAST, NORTH, SOUTH, WEST},
    vector::Vector,
};

type Warehouse = Grid<u8>;
type GpsCoordinate = usize;

pub fn first_answer(input: &str) -> String {
    let (mut warehouse, directions) = warehouse_and_directions(input);
    movements(&mut warehouse, directions, movement);
    gps_coordinates(&warehouse)
        .sum::<GpsCoordinate>()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let (warehouse, directions) = warehouse_and_directions(input);
    let mut wide_warehouse = wide_warehouse(&warehouse);
    println!("{wide_warehouse}");
    movements(&mut wide_warehouse, directions, wide_robot_movement);
    gps_coordinates(&wide_warehouse)
        .sum::<GpsCoordinate>()
        .to_string()
}

fn movements(
    warehouse: &mut Warehouse,
    directions: impl Iterator<Item = Direction>,
    mut movement: impl FnMut(&mut Warehouse, Position, Direction) -> bool,
) {
    let mut robot = robot_position(warehouse);
    for direction in directions {
        if movement(warehouse, robot, direction) {
            robot = robot.add(direction);
        }
        println!("{warehouse}");
    }
}

fn robot_position(warehouse: &mut Grid<u8>) -> Position {
    warehouse
        .find(|_, &element| element == b'@')
        .expect("robot should be present")
        .0
}

fn movement(warehouse: &mut Warehouse, from: Position, direction: Direction) -> bool {
    let to = from.add(direction);
    let element = warehouse[to];
    match element {
        b'#' => false,
        b'O' if !movement(warehouse, to, direction) => false,
        b'.' | b'O' => {
            [warehouse[from], warehouse[to]] = [b'.', warehouse[from]];
            true
        }
        _ => panic!("warehouse should contain only expected elements"),
    }
}

fn wide_warehouse(warehouse: &Warehouse) -> Warehouse {
    let elements = warehouse
        .iter_row_major()
        .flat_map(|(_, &element)| match element {
            b'#' => b"##",
            b'O' => b"[]",
            b'.' => b"..",
            b'@' => b"@.",
            _ => panic!("warehouse should contain only expected elements"),
        })
        .copied()
        .collect_vec();
    Warehouse::from_elements(elements, warehouse.width() * 2)
}

fn wide_robot_movement(warehouse: &mut Warehouse, from: Position, direction: Direction) -> bool {
    let to = from.add(direction);
    let element = warehouse[to];
    match element {
        b'#' => false,
        b'[' if !wide_box_movement(warehouse, to, direction) => false,
        b']' if !wide_box_movement(warehouse, to.add(WEST), direction) => false,
        b'.' | b'[' | b']' => {
            [warehouse[from], warehouse[to]] = [b'.', b'@'];
            true
        }
        _ => panic!("warehouse should contain only expected elements"),
    }
}

fn wide_box_movement(warehouse: &mut Warehouse, west: Position, direction: Direction) -> bool {
    if [EAST, WEST].contains(&direction) {
        let to = west.add(direction);
        let next = to.add(direction);
        let element = warehouse[next];
        match element {
            b'#' => false,
            b'[' if !wide_box_movement(warehouse, next, direction) => false,
            b'.' | b'[' => {
                if direction == EAST {
                    [warehouse[west], warehouse[to], warehouse[next]] = [b'.', b'[', b']'];
                } else {
                    let east = west.add(EAST);
                    [warehouse[to], warehouse[west], warehouse[east]] = [b'[', b']', b'.'];
                }
                true
            }
            _ => panic!("warehouse should contain only expected elements"),
        }
    } else if [NORTH, SOUTH].contains(&direction) {
        if can_vertically_move_boxes(warehouse, west, direction) {
            vertically_move_boxes(warehouse, west, direction);
            true
        } else {
            false
        }
    } else {
        panic!("direction should be north, east, south, or west");
    }
}

fn can_vertically_move_boxes(warehouse: &Warehouse, west: Position, direction: Direction) -> bool {
    let east = west.add(EAST);
    let [to_west, to_east] = [west, east].map(|position| position.add(direction));
    let [west_element, east_element] = [to_west, to_east].map(|position| warehouse[position]);
    match [west_element, east_element] {
        [b'#', _] | [_, b'#'] => false,
        [b'[', b']'] => can_vertically_move_boxes(warehouse, to_west, direction),
        [b']', _] if !can_vertically_move_boxes(warehouse, to_west.add(WEST), direction) => false,
        [_, b'['] if !can_vertically_move_boxes(warehouse, to_east, direction) => false,
        _ => true,
    }
}

fn vertically_move_boxes(warehouse: &mut Warehouse, west: Position, direction: Direction) {
    let east = west.add(EAST);
    let [to_west, to_east] = [west, east].map(|position| position.add(direction));
    let [west_element, east_element] = [to_west, to_east].map(|position| warehouse[position]);
    if west_element == b']' {
        vertically_move_boxes(warehouse, to_west.add(WEST), direction);
    }
    if west_element == b'[' {
        vertically_move_boxes(warehouse, to_west, direction);
    }
    if east_element == b'[' {
        vertically_move_boxes(warehouse, to_east, direction);
    }

    [
        warehouse[west],
        warehouse[east],
        warehouse[to_west],
        warehouse[to_east],
    ] = [b'.', b'.', b'[', b']'];
}

fn gps_coordinates(warehouse: &Warehouse) -> impl Iterator<Item = GpsCoordinate> + use<'_> {
    warehouse
        .iter_row_major()
        .filter(|&(_, &element)| [b'O', b'['].contains(&element))
        .map(|(position, _)| gps_coordinate(position))
}

fn gps_coordinate([row, column]: Position) -> GpsCoordinate {
    100 * GpsCoordinate::conv(row) + GpsCoordinate::conv(column)
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
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1_552_463);
    }

    #[test]
    fn second_answer_example() {
        // test_on_input(DAY, Puzzle::Second, Input::Example(2), 105 + 207 + 306);
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 9021);
    }

    // #[test]
    // fn second_answer_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1200);
    // }
}
