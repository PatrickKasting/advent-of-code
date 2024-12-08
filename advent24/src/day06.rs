use shared::{
    grid::{self, Direction, Grid, Position},
    vector::{RotationInTwoDimensions, Vector},
};

type Map = Grid<char>;

pub fn first_answer(input: &str) -> String {
    let (map, starting_position) = map_and_starting_position(input);
    let visited_positions = visited_positions(&map, starting_position);
    visited_positions
        .iter_row_major()
        .filter(|&(_, &visited)| visited)
        .count()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let (map, starting_position) = map_and_starting_position(input);
    let cycle_obstacles = cycle_obstacles(&map, starting_position);
    cycle_obstacles.len().to_string()
}

fn visited_positions(map: &Map, starting_position: Position) -> Grid<bool> {
    let mut visited = map.map(|_, _| false);
    walk(map, starting_position, grid::NORTH, |position, _| {
        visited[position] = true;
        false
    });
    visited
}

fn cycle_obstacles(map: &Map, starting_position: Position) -> Vec<Position> {
    let mut directions = map.map(|_, _| None);
    let mut cycle_obstacles = vec![];
    walk(
        map,
        starting_position,
        grid::NORTH,
        |position, direction| {
            directions[position] = Some(direction);

            let in_front = position.add(direction);
            if map.get(in_front) == Some(&'.') {
                let mut is_cycle = false;
                walk(map, position, direction.right(), |position, direction| {
                    if directions.get(position) == Some(&Some(direction)) {
                        is_cycle = true;
                        true
                    } else {
                        false
                    }
                });
                if is_cycle {
                    cycle_obstacles.push(in_front);
                }
            }
            false
        },
    );
    cycle_obstacles
}

fn walk(
    map: &Map,
    mut position: Position,
    mut direction: Direction,
    mut stop: impl FnMut(Position, Direction) -> bool,
) {
    while map.get(position).is_some() && !stop(position, direction) {
        let mut next_position;
        loop {
            next_position = position.add(direction);
            if map.get(next_position) != Some(&'#') {
                break;
            }
            direction = direction.right();
        }
        position = next_position;
    }
}

fn map_and_starting_position(input: &str) -> (Map, Position) {
    let mut map = Map::from(input);
    let starting_position = map
        .position(|&char| char == '^')
        .expect("map should have starting position");
    map[starting_position] = '.';
    (map, starting_position)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 6;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 41);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 5516);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 6);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 26_800_609);
    }
}
