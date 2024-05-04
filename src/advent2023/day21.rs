use std::{collections::HashSet, mem};

use crate::data_structures::grid::{self, Coordinate, Grid, Position};

type Map = Grid<char>;

pub fn first(input: &str) -> String {
    let example = Map::from(input);
    number_of_reachable_garden_plots(&example, 64).to_string()
}

pub fn second(_input: &str) -> String {
    unimplemented!()
}

fn number_of_reachable_garden_plots(map: &Map, number_of_steps: usize) -> usize {
    let starting_plot = starting_plot(map);
    let starting_plot_parity = position_parity(starting_plot);

    let mut explored = HashSet::from([starting_plot]);
    let mut frontier = vec![starting_plot];
    let mut next_frontier = vec![];
    let mut is_current_distance_multiple_of_two_from_number_of_steps = number_of_steps % 2 == 0;
    let mut number_of_reachable_garden_plots = 0;
    for _ in 0..=number_of_steps {
        while let Some(plot) = frontier.pop() {
            if position_parity(plot) == starting_plot_parity {
                number_of_reachable_garden_plots += 1;
            }

            for neighbor in grid::neighbors(plot) {
                if map.get(neighbor) == Some(&'.') && explored.insert(neighbor) {
                    next_frontier.push(neighbor);
                }
            }
        }
        mem::swap(&mut frontier, &mut next_frontier);
        is_current_distance_multiple_of_two_from_number_of_steps =
            !is_current_distance_multiple_of_two_from_number_of_steps;
    }
    number_of_reachable_garden_plots
}

fn starting_plot(map: &Map) -> Position {
    map.iter_row_major()
        .find_map(|(position, &tile)| (tile == 'S').then_some(position))
        .expect("there should be exactly one starting position")
}

fn position_parity([row, column]: Position) -> Coordinate {
    (row + column) % 2
}

#[cfg(test)]
mod tests {
    use super::super::tests::{test_on_input, YEAR};
    use crate::{input, Input, Puzzle};

    use super::*;

    const DAY: usize = 21;

    #[test]
    fn first_example() {
        let example = Map::from(&input(YEAR, DAY, Input::Example(0)));
        let actual = number_of_reachable_garden_plots(&example, 6);
        assert_eq!(actual, 16);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 3642);
    }

    // #[test]
    // fn second_example() {
    //     let map = Map::from(&input(YEAR, DAY, Input::Example(0)));
    //     let cases = [6, 10, 50, 100, 500, 1000, 5000]
    //         .into_iter()
    //         .zip_eq([16, 50, 1594, 6536, 167_004, 668_697, 16_733_044]);
    //     test_cases(
    //         |number_of_steps| number_of_reachable_garden_plots(&map, number_of_steps),
    //         cases,
    //     );
    // }
}
