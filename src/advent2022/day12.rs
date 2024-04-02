use crate::{
    data_structures::grid::{Grid, Position},
    search::shortest_path_length,
};

type Height = usize;
type Heightmap = Grid<Height>;

pub fn first(input: &str) -> String {
    let (heightmap, current_position, best_signal) = heightmap(input);
    let from = |position| position == current_position;
    number_of_steps(&heightmap, from, best_signal).to_string()
}

pub fn second(input: &str) -> String {
    let (heightmap, _, best_signal) = heightmap(input);
    let from = |position| heightmap[position] == height('a');
    number_of_steps(&heightmap, from, best_signal).to_string()
}

fn number_of_steps(heightmap: &Heightmap, from: impl Fn(Position) -> bool, to: Position) -> usize {
    let next_squares = |position: Position| {
        position.neighbors().filter(move |&neighbor| {
            heightmap
                .get(neighbor)
                .is_some_and(|&neighbor_height| heightmap[position] <= neighbor_height + 1)
        })
    };
    shortest_path_length(to, |_, _| (), next_squares, from)
        .expect("path from 'E' to 'S' should exist")
}

fn heightmap(input: &str) -> (Heightmap, Position, Position) {
    let mut grid = Grid::from(input);
    let [current_position, best_signal]: [Position; 2] = ['S', 'E'].map(|target| {
        grid.iter_row_major()
            .find_map(|(position, &char)| (char == target).then_some(position))
            .expect("grid should contain 'S' and 'E'")
    });
    grid[current_position] = 'a';
    grid[best_signal] = 'z';
    let heightmap = grid.map(|_, &char| height(char));
    (heightmap, current_position, best_signal)
}

fn height(char: char) -> Height {
    char as Height - 'a' as Height
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 12;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 31);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 472);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 29);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 465);
    }
}
