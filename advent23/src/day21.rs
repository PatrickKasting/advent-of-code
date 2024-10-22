use std::mem;

use ahash::AHashSet;
use easy_cast::Conv;
use shared::grid::{self, Grid, Position};

type Garden = Grid<char>;
type Parity = usize;

const EVEN: Parity = 0;
const ODD: Parity = 1;

pub fn first(input: &str) -> String {
    number_of_reachable_plots_in_exactly(&Garden::from(input), 64).to_string()
}

pub fn second(input: &str) -> String {
    const NUMBER_OF_STEPS: usize = 26_501_365;

    let garden = Garden::from(input);
    debug_assert_repeating_garden_properties(&garden, NUMBER_OF_STEPS);
    let repeats = NUMBER_OF_STEPS / size(&garden);

    let number_of_reachable_plots = number_of_reachable_plots_full(&garden, repeats)
        + number_of_reachable_plots_eighths(&garden, repeats)
        + number_of_reachable_plots_seven_eighths(&garden, repeats)
        + number_of_reachable_plots_tips(&garden);
    number_of_reachable_plots.to_string()
}

fn debug_assert_repeating_garden_properties(garden: &Garden, number_of_steps: usize) {
    let half = size(garden) / 2;

    debug_assert_eq!(
        garden[center(garden)],
        'S',
        "starting position should be center of the garden"
    );

    let middle_row_is_all_plots = garden
        .rows()
        .nth(half)
        .expect("garden should not have size zero")
        .all(|element| ['.', 'S'].contains(element));
    debug_assert!(
        middle_row_is_all_plots,
        "middle row should be all garden plots"
    );
    let middle_column_is_all_plots = garden
        .columns()
        .nth(half)
        .expect("garden should not have size zero")
        .all(|element| ['.', 'S'].contains(element));
    debug_assert!(
        middle_column_is_all_plots,
        "middle column should be all garden plots"
    );

    debug_assert_eq!(
        number_of_steps % size(garden),
        half,
        "gardener should walk to the edge of the garden"
    );
}

fn number_of_reachable_plots_in_exactly(garden: &Garden, number_of_steps: usize) -> usize {
    let starting_plot = center(garden);
    debug_assert_eq!(
        garden[starting_plot], 'S',
        "starting position should be center of the garden"
    );
    let parity = position_parity(starting_plot) ^ (number_of_steps & 1);
    number_of_reachable_plots(garden, starting_plot, parity, number_of_steps)
}

fn center(garden: &Garden) -> Position {
    [Garden::height, Garden::width].map(|dimension| isize::conv(dimension(garden)) / 2)
}

fn position_parity([row, column]: Position) -> Parity {
    usize::conv(row + column) % 2
}

fn number_of_reachable_plots(
    garden: &Garden,
    starting_plot: Position,
    parity: Parity,
    number_of_steps: usize,
) -> usize {
    let mut explored = AHashSet::from([starting_plot]);
    let mut frontier = vec![starting_plot];
    let mut next_frontier = vec![];
    let mut correct_plot_parity = position_parity(starting_plot) == parity;
    let mut number_of_reachable_garden_plots = 0;
    for _ in 0..=number_of_steps {
        while let Some(plot) = frontier.pop() {
            if correct_plot_parity {
                number_of_reachable_garden_plots += 1;
            }

            for neighbor in grid::neighbors(plot) {
                let is_plot = [Some(&'.'), Some(&'S')].contains(&garden.get(neighbor));
                if is_plot && explored.insert(neighbor) {
                    next_frontier.push(neighbor);
                }
            }
        }
        mem::swap(&mut frontier, &mut next_frontier);
        correct_plot_parity = !correct_plot_parity;
    }
    number_of_reachable_garden_plots
}

fn number_of_reachable_plots_full(garden: &Garden, repeats: usize) -> usize {
    let even = number_of_reachable_plots(garden, [0, 0], EVEN, size(garden) * 2);
    let even = repeats.pow(2) * even;

    let odd = number_of_reachable_plots(garden, [0, 0], ODD, size(garden) * 2);
    let odd = (repeats - 1).pow(2) * odd;

    even + odd
}

fn number_of_reachable_plots_eighths(garden: &Garden, repeats: usize) -> usize {
    let sum_of_four_distinct_eights: usize = garden
        .corners_clockwise()
        .into_iter()
        .map(|starting_plot| {
            number_of_reachable_plots(garden, starting_plot, EVEN, size(garden) / 2)
        })
        .sum();
    repeats * sum_of_four_distinct_eights
}

fn number_of_reachable_plots_seven_eighths(garden: &Garden, repeats: usize) -> usize {
    let number_of_steps = size(garden) - 1 + size(garden) / 2;
    let sum_of_four_distinct_seven_eighths: usize = garden
        .corners_clockwise()
        .into_iter()
        .map(|starting_plot| number_of_reachable_plots(garden, starting_plot, ODD, number_of_steps))
        .sum();
    (repeats - 1) * sum_of_four_distinct_seven_eighths
}

fn number_of_reachable_plots_tips(garden: &Garden) -> usize {
    garden
        .edge_midpoints_clockwise()
        .into_iter()
        .map(|starting_plot| {
            number_of_reachable_plots(garden, starting_plot, ODD, size(garden) - 1)
        })
        .sum()
}

fn size(garden: &Garden) -> usize {
    debug_assert_eq!(garden.height(), garden.width(), "garden should be square");
    garden.height()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 21;

    #[test]
    fn first_example() {
        let garden = Garden::from(input(DAY, Input::Example(0)));
        let actual = number_of_reachable_plots_in_exactly(&garden, 6);
        let expected = 16;
        assert_eq!(actual, expected);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 3642);
    }

    #[test]
    fn second_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            608_603_023_105_276_usize,
        );
    }

    #[test]
    fn center() {
        let garden = Garden::from(input(DAY, Input::Example(0)));
        let actual = super::center(&garden);
        let expected = [5, 5];
        assert_eq!(actual, expected);
    }

    #[test]
    fn number_of_reachable_plots_full() {
        let garden = Garden::from(input(DAY, Input::Example(1)));
        let actual = super::number_of_reachable_plots_full(&garden, 2);
        let expected = 24;
        assert_eq!(actual, expected);
    }

    #[test]
    fn number_of_reachable_plots_eighths() {
        let garden = Garden::from(input(DAY, Input::Example(1)));
        let actual = super::number_of_reachable_plots_eighths(&garden, 2);
        let expected = 8;
        assert_eq!(actual, expected);
    }

    #[test]
    fn number_of_reachable_plots_seven_eighths() {
        let garden = Garden::from(input(DAY, Input::Example(1)));
        let actual = super::number_of_reachable_plots_seven_eighths(&garden, 2);
        let expected = 16;
        assert_eq!(actual, expected);
    }

    #[test]
    fn number_of_reachable_plots_tips() {
        let garden = Garden::from(input(DAY, Input::Example(1)));
        let actual = super::number_of_reachable_plots_tips(&garden);
        let expected = 16;
        assert_eq!(actual, expected);
    }
}
