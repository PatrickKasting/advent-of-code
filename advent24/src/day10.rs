use shared::{
    grid::{self, Grid, Position},
    search,
};

type TopographicMap = Grid<usize>;

pub fn first_answer(input: &str) -> String {
    let map = TopographicMap::from(input);
    map.iter_row_major()
        .filter(|&(_, &height)| height == 0)
        .map(|(start, _)| score(&map, start))
        .sum::<usize>()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let map = TopographicMap::from(input);
    todo!()
}

fn score(map: &TopographicMap, start: Position) -> usize {
    let mut exploration = search::Exploration::new([]);
    exploration.explore(start, |position| {
        grid::neighbors(position)
            .into_iter()
            .filter(move |&neighbor| {
                map.get(neighbor)
                    .is_some_and(|&height| height.wrapping_sub(map[position]) == 1)
            })
    });
    exploration
        .explored()
        .into_iter()
        .filter(|&position| map[position] == 9)
        .count()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 10;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 36);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 786);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 34);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1200);
    }
}
