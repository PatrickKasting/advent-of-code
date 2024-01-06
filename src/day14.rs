use std::collections::HashMap;

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::grid::{Direction, Grid, Position};

type Platform = Grid<char>;

fn sorted_round_rock_positions(platform: &Platform, direction: Direction) -> Vec<Position> {
    let platform_elements: Box<dyn Iterator<Item = (Position, &char)>> = match direction {
        Direction::North | Direction::South => Box::new(platform.iter_row_major()),
        Direction::West | Direction::East => Box::new(platform.iter_column_major()),
    };
    let mut round_rock_positions = platform_elements
        .filter_map(|(position, &element)| (element == 'O').then_some(position))
        .collect_vec();
    if [Direction::South, Direction::East].contains(&direction) {
        round_rock_positions.reverse();
    }
    round_rock_positions
}

fn tilt(platform: &mut Platform, direction: Direction) {
    for rock_position in sorted_round_rock_positions(platform, direction) {
        platform[rock_position] = '.';
        let mut current_position = rock_position;
        loop {
            let next_position = current_position.neighbor(direction);
            if !platform
                .get(next_position)
                .is_some_and(|&space| space == '.')
            {
                break;
            }
            current_position = next_position;
        }
        platform[current_position] = 'O';
    }
}

fn cycle(platform: &mut Platform, number_of_cycles: usize) {
    for _ in 0..number_of_cycles {
        for direction in Direction::iter() {
            tilt(platform, direction);
        }
    }
}

fn cycle_start_and_length(platform: &mut Platform) -> (usize, usize) {
    let mut previous = HashMap::new();
    for number_of_cycles in 0usize.. {
        previous.insert(platform.clone(), number_of_cycles);
        cycle(platform, 1);
        if let Some(&cycle_start) = previous.get(platform) {
            return (cycle_start, 1 + number_of_cycles - cycle_start);
        }
    }
    unreachable!("cycle should occur");
}

fn total_load(platform: &Platform) -> usize {
    (1..)
        .zip(platform.rows().rev())
        .map(|(index, row)| index * row.filter(|&&space| space == 'O').count())
        .sum()
}

pub fn first(input: String) -> String {
    let mut platform = Platform::from(&input[..]);
    tilt(&mut platform, Direction::North);
    total_load(&platform).to_string()
}

pub fn second(input: String) -> String {
    let mut platform = Platform::from(&input[..]);
    let (cycle_start, cycle_length) = cycle_start_and_length(&mut platform);
    let number_of_missing_cycles = (1_000_000_000 - cycle_start) % cycle_length;
    cycle(&mut platform, number_of_missing_cycles);
    total_load(&platform).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{input, tests::*, Input, Puzzle};

    use super::{cycle, Platform};

    const DAY: usize = 14;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 136);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::Real, 108759);
    }

    fn after_cycles(number_of_cycles: usize) -> Platform {
        let mut platform = Platform::from(&input(DAY, Input::Example(0))[..]);
        cycle(&mut platform, number_of_cycles);
        platform
    }

    fn test_after_cycles(number_of_cycles: usize, expected: &str) {
        assert_eq!(after_cycles(number_of_cycles), Platform::from(expected));
    }

    #[test]
    fn after_one_cycle() {
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
        test_after_cycles(1, expected);
    }

    #[test]
    fn after_two_cycles() {
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
        test_after_cycles(2, expected);
    }

    #[test]
    fn after_three_cycles() {
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
        test_after_cycles(3, expected);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 64);
    }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::Real, 33438);
    // }
}
