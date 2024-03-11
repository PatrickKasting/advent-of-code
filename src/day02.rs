use std::cmp;

use itertools::Itertools;
use regex::Regex;

use crate::utilities::number;

const NUMBER_OF_COLORS: usize = 3;

type Count = usize;
type Counts = [Count; NUMBER_OF_COLORS];
type Game = Vec<Counts>;

fn counts(handful: &str) -> Counts {
    let count_and_color = Regex::new(r"(\d+) (red|green|blue)").expect("regex should be valid");
    let captures = count_and_color
        .captures_iter(handful)
        .map(|capture| capture.extract());

    let [mut red, mut green, mut blue] = [0, 0, 0];
    for (_, [count, color]) in captures {
        let count: Count = number(count);
        match color {
            "red" => red += count,
            "green" => green += count,
            "blue" => blue += count,
            _ => panic!("color should be 'red', 'green', or 'blue'"),
        }
    }
    [red, green, blue]
}

fn game(line: &str) -> Game {
    let colon_index = line.find(':').expect("every line should contain a colon");
    let handfuls = &line[colon_index + 1..];
    handfuls.split(';').map(counts).collect_vec()
}

fn games(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.lines().map(game)
}

const BAG_COUNTS: Counts = [12, 13, 14];

fn are_counts_possible(counts: Counts) -> bool {
    counts
        .into_iter()
        .zip(BAG_COUNTS)
        .all(|(count, bag_count)| count <= bag_count)
}

fn is_game_possible(game: &Game) -> bool {
    game.iter().copied().all(are_counts_possible)
}

fn ids_of_possible_games(games: impl Iterator<Item = Game>) -> impl Iterator<Item = usize> {
    (1..)
        .zip(games)
        .filter(|(_, game)| is_game_possible(game))
        .map(|(game_number, _)| game_number)
}

fn maximum_counts(game: Game) -> Counts {
    let mut maximums = [0; NUMBER_OF_COLORS];
    for count in game {
        for (max, count) in maximums.iter_mut().zip(count) {
            *max = cmp::max(*max, count);
        }
    }
    maximums
}

fn power(counts: Counts) -> Count {
    counts.into_iter().product()
}

fn power_of_minimum_set(game: Game) -> Count {
    power(maximum_counts(game))
}

pub fn first(input: &str) -> String {
    ids_of_possible_games(games(input))
        .sum::<usize>()
        .to_string()
}

pub fn second(input: &str) -> String {
    games(input)
        .map(power_of_minimum_set)
        .sum::<Count>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, Input, Puzzle};

    const DAY: usize = 2;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 8);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 2149);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 2286);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 71274);
    }
}
