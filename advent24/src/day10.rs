use shared::{
    grid::{self, Grid, Position},
    search,
};

type TopographicMap = Grid<Height>;
type Height = usize;
type Score = usize;
type Rating = usize;

pub fn first_answer(input: &str) -> String {
    let map = TopographicMap::from(input);
    trailheads(&map)
        .map(|trailhead| score(&map, trailhead))
        .sum::<usize>()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let map = TopographicMap::from(input);
    let ratings = ratings(&map);
    positions_with(&map, 0)
        .map(|trailhead| ratings[trailhead])
        .sum::<usize>()
        .to_string()
}

fn score(map: &TopographicMap, trailhead: Position) -> Score {
    let mut exploration = search::Exploration::new([]);
    let successors = |position| {
        grid::neighbors(position)
            .into_iter()
            .filter(move |&neighbor| {
                map.get(neighbor)
                    .is_some_and(|&height| height.wrapping_sub(map[position]) == 1)
            })
    };
    exploration.explore(trailhead, successors);
    exploration
        .explored()
        .into_iter()
        .filter(|&position| map[position] == 9)
        .count()
}

fn ratings(map: &TopographicMap) -> Grid<Rating> {
    let mut ratings = map.map(|_, &height| usize::from(height == 9));
    for height in (0..9).rev() {
        for position in positions_with(map, height) {
            let previous = grid::neighbors(position).into_iter().filter(|&neighbor| {
                map.get(neighbor)
                    .is_some_and(|&previous_height| previous_height.wrapping_sub(height) == 1)
            });
            ratings[position] = previous.map(|previous| ratings[previous]).sum();
        }
    }
    ratings
}

fn trailheads(map: &Grid<usize>) -> impl Iterator<Item = Position> + use<'_> {
    positions_with(map, 0)
}

fn positions_with(map: &Grid<usize>, height: usize) -> impl Iterator<Item = Position> + use<'_> {
    map.iter_row_major()
        .filter(move |&(_, &actual)| actual == height)
        .map(|(target_position, _)| target_position)
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
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 81);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1722);
    }
}
