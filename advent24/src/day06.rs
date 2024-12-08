use shared::{
    grid::{self, Grid, Position},
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
    todo!()
}

fn visited_positions(map: &Map, starting_position: [isize; 2]) -> Grid<bool> {
    let mut visited = map.map(|_, _| false);
    let mut position = starting_position;
    let mut direction = grid::NORTH;
    while map.get(position).is_some() {
        visited[position] = true;

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
    visited
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
