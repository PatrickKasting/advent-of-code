use easy_cast::Conv;
use itertools::Itertools;
use shared::{
    grid::{self, Direction, EAST, Grid, NORTH, Position, SOUTH, WEST},
    vector::Vector,
};

type Warehouse = Grid<u8>;
type GpsCoordinate = usize;

pub fn first_answer(input: &str) -> String {
    let (mut warehouse, directions) = warehouse_and_directions(input);
    moves(&mut warehouse, directions);
    gps_coordinates(&warehouse)
        .sum::<GpsCoordinate>()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let (warehouse, directions) = warehouse_and_directions(input);
    let mut wide_warehouse = wide_warehouse(&warehouse);
    moves(&mut wide_warehouse, directions);
    gps_coordinates(&wide_warehouse)
        .sum::<GpsCoordinate>()
        .to_string()
}

fn moves(warehouse: &mut Warehouse, directions: impl Iterator<Item = Direction>) {
    let mut robot = robot_position(warehouse);
    for direction in directions {
        if r#move(warehouse, robot, direction) {
            robot = robot.add(direction);
        }
    }
}

fn robot_position(warehouse: &mut Grid<u8>) -> Position {
    warehouse
        .find(|_, &element| element == b'@')
        .expect("robot should be present")
        .0
}

fn r#move(warehouse: &mut Warehouse, from: Position, direction: Direction) -> bool {
    let to = from.add(direction);
    let element = warehouse[to];
    let is_possible = match element {
        b'#' => false,
        b'O' => r#move(warehouse, to, direction),
        b'[' => move_wide_box(warehouse, to, direction),
        b']' => move_wide_box(warehouse, to.add(WEST), direction),
        b'.' => true,
        _ => panic!("warehouse should contain only expected elements"),
    };
    if is_possible {
        [warehouse[from], warehouse[to]] = [b'.', warehouse[from]];
    }
    is_possible
}

fn move_wide_box(warehouse: &mut Warehouse, west: Position, direction: Direction) -> bool {
    let mut boxes = vec![];
    let is_possible = match direction {
        EAST | WEST => wide_boxes_east_west(&mut boxes, warehouse, west, direction),
        NORTH | SOUTH => wide_boxes_north_south(&mut boxes, warehouse, west, direction),
        _ => panic!("direction should be orthogonal"),
    };
    if !is_possible {
        return false;
    }
    for &west in &boxes {
        let to_west = west.add(direction);
        let [east, to_east] = [west, to_west].map(|p| p.add(EAST));
        [warehouse[west], warehouse[east]] = [b'.', b'.'];
        [warehouse[to_west], warehouse[to_east]] = [b'[', b']'];
    }
    true
}

fn wide_boxes_east_west(
    boxes: &mut Vec<Position>,
    warehouse: &mut Grid<u8>,
    west: Position,
    direction: Direction,
) -> bool {
    let to_west = west.add(direction);
    let next = to_west.add(direction);
    let neighbor = to_west.add(direction.mul(isize::from(direction == EAST)));
    let element = warehouse[neighbor];
    let result = match element {
        b'#' => false,
        b'[' | b']' => wide_boxes_east_west(boxes, warehouse, next, direction),
        b'.' => true,
        _ => panic!("wide warehouse should contain only expected elements"),
    };
    boxes.push(west);
    result
}

fn wide_boxes_north_south(
    boxes: &mut Vec<Position>,
    warehouse: &mut Grid<u8>,
    west: Position,
    direction: Direction,
) -> bool {
    let east = west.add(EAST);
    let [to_west, to_east] = [west, east].map(|p| p.add(direction));
    let [west_element, east_element] = [to_west, to_east].map(|p| warehouse[p]);
    let result = match [west_element, east_element] {
        [b'#', b'.' | b'[' | b'#'] | [b'.' | b']', b'#'] => false,
        [b'[', b']'] => wide_boxes_north_south(boxes, warehouse, to_west, direction),
        [b']', b'['] => {
            wide_boxes_north_south(boxes, warehouse, to_west.add(WEST), direction)
                && wide_boxes_north_south(boxes, warehouse, to_east, direction)
        }
        [b']', b'.'] => wide_boxes_north_south(boxes, warehouse, to_west.add(WEST), direction),
        [b'.', b'['] => wide_boxes_north_south(boxes, warehouse, to_east, direction),
        [b'.', b'.'] => true,
        _ => panic!("wide warehouse should contain only expected elements"),
    };
    if !boxes.contains(&west) {
        boxes.push(west);
    }
    result
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
        test_on_input(DAY, Puzzle::Second, Input::Example(2), 105 + 207 + 306);
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 9021);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1_554_058);
    }
}
