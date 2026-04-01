use easy_cast::Cast;
use shared::{
    grid::{Grid, Position, orthogonal_neighbors},
    search,
    string::isizes,
};

type MemorySpace = Grid<u8>;

const MEMORY_SPACE_SIZE: usize = 70 + 1;

pub fn first_answer(input: &str) -> String {
    let mut memory_space = uncorrupted_memory_space();
    for position in falling_byte_positions(input).take(1024) {
        memory_space[position] = b'#';
    }
    shortest_path_length(&memory_space)
        .expect("path from top left to bottom right should exist")
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let mut memory_space = uncorrupted_memory_space();
    for position in falling_byte_positions(input) {
        memory_space[position] = b'#';
        if shortest_path_length(&memory_space).is_none() {
            return format!("{},{}", position[1], position[0]);
        }
    }
    unreachable!("a falling byte should prevent reaching the exit");
}

fn shortest_path_length(memory_space: &MemorySpace) -> Option<usize> {
    let source = [0; 2];
    let successors = |position| {
        orthogonal_neighbors(position)
            .into_iter()
            .filter(|&neighbor| {
                memory_space
                    .get(neighbor)
                    .is_some_and(|&element| element == b'.')
            })
    };
    let target = |position| position == [(memory_space.width() - 1).cast(); 2];
    search::shortest_path_length(source, successors, target)
}

fn falling_byte_positions(input: &str) -> impl Iterator<Item = Position> {
    input.lines().map(|line| {
        let coordinates = isizes(line);
        [coordinates[1], coordinates[0]]
    })
}

fn uncorrupted_memory_space() -> MemorySpace {
    MemorySpace::new(MEMORY_SPACE_SIZE, MEMORY_SPACE_SIZE, |_| b'.')
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 18;

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 372);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, "25,6");
    }
}
