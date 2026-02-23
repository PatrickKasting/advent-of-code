use ahash::AHashSet;
use shared::{
    grid::{self, Direction, Grid, Position, DIAGONAL_DIRECTIONS},
    search,
    vector::Vector,
};

type Map = Grid<Plant>;
type Plant = char;
type Price = usize;

pub fn first_answer(input: &str) -> String {
    let map = Map::from(input);
    total_original_price(&map).to_string()
}

pub fn second_answer(input: &str) -> String {
    let map = Map::from(input);
    total_bulk_price(&map).to_string()
}

fn total_original_price(map: &Map) -> Price {
    total_price(map, |area| {
        let perimeter: usize = area
            .iter()
            .map(|&position| number_of_edges(map, position))
            .sum();
        area.len() * perimeter
    })
}

fn total_bulk_price(map: &Map) -> Price {
    total_price(map, |area| {
        let number_of_edges: usize = area
            .iter()
            .map(|&position| number_of_corners(map, position))
            .sum();
        area.len() * number_of_edges
    })
}

fn total_price(map: &Map, mut area_price: impl FnMut(&AHashSet<Position>) -> Price) -> Price {
    let mut total_price = 0;
    let mut fenced = map.map(|_, _| false);
    while let Some((next, _)) = fenced.find(|_, &fenced| !fenced) {
        let area = area(map, next);
        total_price += area_price(&area);
        for position in area {
            fenced[position] = true;
        }
    }
    total_price
}

fn area(map: &Map, source: Position) -> AHashSet<Position> {
    let plant = map[source];
    let mut exploration = search::Exploration::new([]);
    let successors = |position| {
        grid::orthogonal_neighbors(position)
            .into_iter()
            .filter(|&neighbor| plant_matches(map, neighbor, plant))
    };
    exploration.explore(source, successors);
    exploration.explored()
}

fn number_of_edges(map: &Map, position: Position) -> usize {
    let plant = map[position];
    let successors = grid::orthogonal_neighbors(position)
        .into_iter()
        .filter(|&neighbor| plant_matches(map, neighbor, plant))
        .count();
    4 - successors
}

fn number_of_corners(map: &Map, position: Position) -> usize {
    DIAGONAL_DIRECTIONS
        .into_iter()
        .filter(|&direction| is_corner(map, position, direction))
        .count()
}

fn is_corner(map: &Map, position: Position, direction: Direction) -> bool {
    let plant = map[position];
    let orthogonals = [[direction[0], 0], [0, direction[1]]]
        .map(|orthogonal| position.add(orthogonal))
        .map(|neighbor| plant_matches(map, neighbor, plant));
    let diagonal = plant_matches(map, position.add(direction), plant);
    let inward = !orthogonals[0] && !orthogonals[1];
    let outward = orthogonals[0] && orthogonals[1] && !diagonal;
    inward || outward
}

fn plant_matches(map: &Map, position: Position, expected: Plant) -> bool {
    map.get(position).is_some_and(|&actual| actual == expected)
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
    fn second_answer_examples() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 80);
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 436);
        test_on_input(DAY, Puzzle::Second, Input::Example(3), 236);
        test_on_input(DAY, Puzzle::Second, Input::Example(4), 368);
        test_on_input(DAY, Puzzle::Second, Input::Example(2), 1206);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 881_182);
    }
}
