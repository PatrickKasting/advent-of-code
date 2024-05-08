use itertools::Itertools;

use crate::{
    data_structures::grid::{self, Direction, Grid, Position},
    vector::Vector,
    HashMap,
};

type Platform = Grid<char>;

pub fn first(input: &str) -> String {
    let mut platform = Platform::from(input);
    tilt(&mut platform, grid::NORTH);
    total_load(&platform).to_string()
}

pub fn second(input: &str) -> String {
    let mut platform = Platform::from(input);
    let (cycle_start, cycle_length) = cycle_start_and_length(&mut platform);
    let number_of_missing_cycles = (1_000_000_000 - cycle_start) % cycle_length;
    cycles(&mut platform, number_of_missing_cycles);
    total_load(&platform).to_string()
}

fn total_load(platform: &Platform) -> usize {
    (1..)
        .zip(platform.rows().rev())
        .map(|(index, row)| index * row.filter(|&&space| space == 'O').count())
        .sum()
}

fn cycle_start_and_length(platform: &mut Platform) -> (usize, usize) {
    let mut previous = HashMap::new();
    for number_of_cycles in 0_usize.. {
        previous.insert(platform.clone(), number_of_cycles);
        cycles(platform, 1);
        if let Some(&cycle_start) = previous.get(platform) {
            return (cycle_start, 1 + number_of_cycles - cycle_start);
        }
    }
    unreachable!("cycle should occur");
}

fn cycles(platform: &mut Platform, number_of_cycles: usize) {
    let directions = [grid::NORTH, grid::WEST, grid::SOUTH, grid::EAST];
    for _ in 0..number_of_cycles {
        for direction in directions {
            tilt(platform, direction);
        }
    }
}

fn tilt(platform: &mut Platform, direction: Direction) {
    for mut rock_position in sorted_round_rock_positions(platform, direction) {
        platform[rock_position] = '.';
        loop {
            let next_position = rock_position.add(direction);
            if platform.get(next_position) != Some(&'.') {
                break;
            }
            rock_position = next_position;
        }
        platform[rock_position] = 'O';
    }
}

fn sorted_round_rock_positions(platform: &Platform, direction: Direction) -> Vec<Position> {
    let platform_elements: Box<dyn Iterator<Item = (Position, &char)>> = match direction {
        grid::NORTH | grid::SOUTH => Box::new(platform.iter_row_major()),
        grid::WEST | grid::EAST => Box::new(platform.iter_column_major()),
        _ => panic!("direction should be one of four unit vectors"),
    };
    let mut round_rock_positions = platform_elements
        .filter_map(|(position, &element)| (element == 'O').then_some(position))
        .collect_vec();
    if [grid::SOUTH, grid::EAST].contains(&direction) {
        round_rock_positions.reverse();
    }
    round_rock_positions
}

#[cfg(test)]
mod tests {
    use super::super::tests::{test_on_input, YEAR};
    use crate::{Input, Puzzle};

    use super::{cycles, Platform};

    const DAY: usize = 14;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 136);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 108_759);
    }

    fn platform_after_cycles(input: Input, number_of_cycles: usize) -> Platform {
        let mut platform = Platform::from(&crate::input(YEAR, DAY, input)[..]);
        cycles(&mut platform, number_of_cycles);
        platform
    }

    fn test_platform_after_cycles(input: Input, number_of_cycles: usize, expected: &str) {
        assert_eq!(
            platform_after_cycles(input, number_of_cycles),
            Platform::from(expected)
        );
    }

    #[test]
    fn platform_after_one_cycle() {
        let expected = "\
            .....#....\n\
            ....#...O#\n\
            ...OO##...\n\
            .OO#......\n\
            .....OOO#.\n\
            .O#...O#.#\n\
            ....O#....\n\
            ......OOOO\n\
            #...O###..\n\
            #..OO#....\n\
        ";
        test_platform_after_cycles(Input::Example(0), 1, expected);
    }

    #[test]
    fn platform_after_two_cycles() {
        let expected = "\
            .....#....\n\
            ....#...O#\n\
            .....##...\n\
            ..O#......\n\
            .....OOO#.\n\
            .O#...O#.#\n\
            ....O#...O\n\
            .......OOO\n\
            #..OO###..\n\
            #.OOO#...O\n\
        ";
        test_platform_after_cycles(Input::Example(0), 2, expected);
    }

    #[test]
    fn platform_after_three_cycles() {
        let expected = "\
            .....#....\n\
            ....#...O#\n\
            .....##...\n\
            ..O#......\n\
            .....OOO#.\n\
            .O#...O#.#\n\
            ....O#...O\n\
            .......OOO\n\
            #...O###.O\n\
            #.OOO#...O\n\
        ";
        test_platform_after_cycles(Input::Example(0), 3, expected);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 64);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 89089);
    }
}
