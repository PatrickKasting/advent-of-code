use ahash::AHashSet;
use itertools::Itertools;
use shared::{
    grid::{self, Grid, Position},
    search,
};

type Map = Grid<char>;
type Price = usize;

pub fn first_answer(input: &str) -> String {
    let map = Map::from(input);
    total_price(&map).to_string()
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn total_price(map: &Map) -> Price {
    let mut total_price = 0;
    let mut fenced = map.map(|_, _| false);
    while let Some((new, _)) = fenced.find(|_, &fenced| !fenced) {
        let (area, perimeter) = area_and_perimeter(map, new);
        total_price += area.len() * perimeter;
        for position in area {
            fenced[position] = true;
        }
    }
    total_price
}

fn area_and_perimeter(map: &Map, source: Position) -> (AHashSet<Position>, usize) {
    let plant = map[source];
    let mut exploration = search::Exploration::new([]);
    let mut perimeter = 0;
    let successors = |position| {
        let successors = grid::neighbors(position)
            .into_iter()
            .filter(|&neighbor| map.get(neighbor).is_some_and(|&next| next == plant))
            .collect_vec();
        perimeter += 4 - successors.len();
        successors
    };
    exploration.explore(source, successors);
    (exploration.explored(), perimeter)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 12;

    #[test]
    fn first_answer_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 140);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 772);
        test_on_input(DAY, Puzzle::First, Input::Example(2), 1930);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1_467_094);
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
