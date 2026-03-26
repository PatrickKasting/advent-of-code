use shared::{
    grid::{Direction, Grid, Position, EAST},
    search,
    vector::{RotationInTwoDimensions, Vector},
};

type Maze = Grid<u8>;

pub fn first_answer(input: &str) -> String {
    let maze = Maze::from(input);
    let [start_tile, end_tile] = [start_tile, end_tile].map(|f| f(&maze));

    let source = (start_tile, EAST);
    let successors = |(tile, direction): (Position, Direction)| {
        let mut successors = vec![
            ((tile, direction.left()), 1000),
            ((tile, direction.right()), 1000),
        ];
        let next_tile = tile.add(direction);
        if maze[next_tile] != b'#' {
            successors.push(((next_tile, direction), 1_usize));
        }
        successors
    };
    let target = |(tile, _)| tile == end_tile;
    search::minimum_path_cost(source, successors, target)
        .expect("path from start tile to end tile should exist")
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let maze = Maze::from(input);
    todo!()
}

fn start_tile(maze: &Maze) -> Position {
    maze.find(|_, &element| element == b'S')
        .expect("maze should have a start tile")
        .0
}

fn end_tile(maze: &Maze) -> Position {
    maze.find(|_, &element| element == b'E')
        .expect("maze should have an end tile")
        .0
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 16;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 7036);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 11048);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 135512);
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
