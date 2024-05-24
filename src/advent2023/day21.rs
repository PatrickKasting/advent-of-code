use std::mem;

use easy_cast::{Cast, Conv};

use crate::{
    data_structures::grid::{self, Grid, Position},
    HashSet,
};

type Map = Grid<char>;
type Parity = bool;

const EVEN: Parity = true;
const ODD: Parity = false;

pub fn first(input: &str) -> String {
    let map = Map::from(input);
    number_of_reachable_plots(&map, start(&map), EVEN, 64).to_string()
}

pub fn second(input: &str) -> String {
    number_of_reachable_plots_infinite_map(&Map::from(input), 26_501_365).to_string()
}

fn number_of_reachable_plots_infinite_map(map: &Map, number_of_steps: usize) -> usize {
    validate(map);

    let map_radius = div_ceil(map.height(), 2);

    let tips: [Position; 4] = [
        [isize::conv(map.height()) - 1, map_radius.cast()],
        [map_radius.cast(), 0],
        [0, map_radius.cast()],
        [map_radius.cast(), isize::conv(map.width()) - 1],
    ];
    let tip_parity = ((number_of_steps + map_radius) / map.height()) % 2 == 0;
    let extra_steps = (number_of_steps + map_radius) % map.height();
    tips.map(|start| number_of_reachable_plots(map, start, tip_parity, extra_steps))
        .into_iter()
        .sum()

    // let full_even = number_of_reachable_plots(map, start(map), EVEN, usize::MAX);
    // let full_odd = number_of_reachable_plots(map, start(map), ODD, usize::MAX);
    // let corner_even = map
    //     .corners_clockwise()
    //     .map(|corner| number_of_reachable_plots(map, corner, EVEN, map.height() / 2 - 1));
    // let corner_odd = map
    //     .corners_clockwise()
    //     .map(|corner| number_of_reachable_plots(map, corner, ODD, map.height() / 2 - 1));

    // let radius_in_map_sizes = number_of_steps / map.height();
    // let ([full_edge, full_non_edge], [corner_edge, corner_non_edge]) =
    //     if radius_in_map_sizes % 2 == number_of_steps % 2 {
    //         ([full_odd, full_even], [corner_odd, corner_even])
    //     } else {
    //         ([full_even, full_odd], [corner_even, corner_odd])
    //     };

    // let edge = full_edge * (radius_in_map_sizes + 1) * (radius_in_map_sizes + 1)
    //     - corner_edge.into_iter().sum::<usize>() * (radius_in_map_sizes + 1);
    // let non_edge = full_non_edge * (radius_in_map_sizes) * (radius_in_map_sizes)
    //     + corner_non_edge.into_iter().sum::<usize>() * (radius_in_map_sizes);
    // edge + non_edge
}

fn validate(map: &Grid<char>) {
    let start = start(map);

    assert_eq!(map.height(), map.width(), "map should be square");
    assert!(map.height() % 2 == 1, "map size should be odd");
    assert_eq!(
        start,
        [isize::conv(map.height()) / 2, isize::conv(map.width()) / 2],
        "starting plot should be in the middle of the map"
    );
    assert!(
        (0..map.height().cast()).all(|row| ".S".contains(map[[row, start[1]]])),
        "starting row should be clear"
    );
    assert!(
        (0..map.width().cast()).all(|column| ".S".contains(map[[start[0], column]])),
        "starting column should be clear"
    );
}

fn number_of_reachable_plots(
    map: &Map,
    start: Position,
    mut parity: Parity,
    number_of_steps: usize,
) -> usize {
    let mut explored = HashSet::from([start]);
    let mut frontier = vec![start];
    let mut next_frontier = vec![];
    let mut number_of_reachable_garden_plots = 0;
    for _ in 0..=number_of_steps {
        if frontier.is_empty() {
            break;
        }
        while let Some(plot) = frontier.pop() {
            if parity {
                number_of_reachable_garden_plots += 1;
            }

            for neighbor in grid::neighbors(plot) {
                if map.get(neighbor) == Some(&'.') && explored.insert(neighbor) {
                    next_frontier.push(neighbor);
                }
            }
        }
        mem::swap(&mut frontier, &mut next_frontier);
        parity = !parity;
    }
    number_of_reachable_garden_plots
}

fn start(map: &Map) -> Position {
    map.iter_row_major()
        .find_map(|(position, &tile)| (tile == 'S').then_some(position))
        .expect("there should be exactly one starting position")
}

fn div_ceil(dividend: usize, divisor: usize) -> usize {
    (dividend + divisor - 1) / divisor
}

#[cfg(test)]
mod tests {
    use super::super::tests::{test_on_input, YEAR};
    use crate::{input, tests::test_cases, Input, Puzzle};

    use super::*;

    const DAY: usize = 21;

    #[test]
    fn first_example() {
        let map = Map::from(&input(YEAR, DAY, Input::Example(0)));
        let actual = number_of_reachable_plots(&map, start(&map), EVEN, 6);
        assert_eq!(actual, 16);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 3642);
    }

    #[test]
    fn second_example() {
        let map = Map::from(&input(YEAR, DAY, Input::Example(1)));
        let function =
            |number_of_steps| number_of_reachable_plots_infinite_map(&map, number_of_steps);
        let cases = [
            (7, 52),
            (8, 68),
            (25, 576),
            (42, 1576),
            (59, 3068),
            (76, 5052),
            (1180148, 1185525742508usize),
        ];
        test_cases(function, cases);
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
}
