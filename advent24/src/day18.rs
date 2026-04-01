use easy_cast::Cast;
use shared::{
    grid::{Grid, orthogonal_neighbors},
    search,
    string::isizes,
};

type MemorySpace = Grid<u8>;

const MEMORY_SPACE_SIZE: usize = 70 + 1;
const NUMBER_OF_FALLING_BYTES: usize = 1024;

pub fn first_answer(input: &str) -> String {
    let memory_space = memory_space(input);
    minimum_number_of_steps(&memory_space).to_string()
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn minimum_number_of_steps(memory_space: &MemorySpace) -> usize {
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
        .expect("path from top left to bottom right should exist")
}

fn memory_space(input: &str) -> MemorySpace {
    let mut memory_space = MemorySpace::new(MEMORY_SPACE_SIZE, MEMORY_SPACE_SIZE, |_| b'.');
    for line in input.lines().take(NUMBER_OF_FALLING_BYTES) {
        let coordinates = isizes(line);
        memory_space[[coordinates[1], coordinates[0]]] = b'#';
    }
    memory_space
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

    // #[test]
    // fn second_answer_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 34);
    // }

    // #[test]
    // fn second_answer_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1200);
    // }
}
