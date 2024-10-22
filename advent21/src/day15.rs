use easy_cast::{Cast, Conv};
use shared::{
    grid::{neighbors, Grid, Position},
    search::minimum_path_cost,
    vector::Vector,
};

type Cave = Grid<RiskLevel>;
type RiskLevel = isize;

pub fn first_answer(input: &str) -> String {
    let cave = Cave::from(input);
    lowest_total_risk(&cave, 1).to_string()
}

pub fn second_answer(input: &str) -> String {
    let cave = Cave::from(input);
    lowest_total_risk(&cave, 5).to_string()
}

fn lowest_total_risk(cave: &Cave, expansion_factor: isize) -> RiskLevel {
    debug_assert!(cave.height() == cave.width(), "cave should be square");

    let expanded_cave_size = isize::conv(cave.width()) * expansion_factor;
    let [start, goal] = [[0, 0], [expanded_cave_size - 1, expanded_cave_size - 1]];
    let successors = |position| {
        neighbors(position).into_iter().filter_map(|neighbor| {
            extended_cave_risk(cave, expansion_factor, neighbor).map(|risk| (neighbor, risk))
        })
    };
    minimum_path_cost(start, successors, |position| position == goal)
        .expect("path from start to goal should exist")
}

fn extended_cave_risk(
    cave: &Cave,
    expansion_factor: isize,
    position @ [row, column]: Position,
) -> Option<RiskLevel> {
    let original_cave_size: isize = cave.width().cast();
    let expanded_cave_size = original_cave_size * expansion_factor;
    if row < 0 || expanded_cave_size <= row || column < 0 || expanded_cave_size <= column {
        return None;
    }

    let [tile_row, tile_column] = position.div(original_cave_size);
    let original_position = position.map(|coordinate| coordinate % original_cave_size);
    let &original_risk = cave
        .get(original_position)
        .expect("modulo arithmetic should ensure that position is within original grid");
    Some((original_risk + tile_row + tile_column - 1) % 9 + 1)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 15;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 40);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 652);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 315);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 2938);
    }
}
