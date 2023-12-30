use std::cmp;

use itertools::Itertools;
use regex::Regex;

use crate::utilities::number;

type Count = usize;
const NUMBER_OF_COLORS: usize = 3;
type Counts = [Count; NUMBER_OF_COLORS];
type Game = Vec<Counts>;

fn counts(handful: &str) -> Counts {
    let count_and_color =
        Regex::new(r"(?<count>\d+) (?<color>red|green|blue)").expect("regex should be valid");
    let captures = count_and_color.captures_iter(handful);

    let [mut red, mut green, mut blue] = [0, 0, 0];
    for capture in captures {
        let count: Count = number(
            capture
                .name("count")
                .expect("every match should contain an amount")
                .as_str(),
        );
        match capture
            .name("color")
            .expect("every match should contain a color")
            .as_str()
        {
            "red" => red += count,
            "green" => green += count,
            "blue" => blue += count,
            _ => unreachable!("color should be 'red', 'green', or 'blue'"),
        }
    }

    [red, green, blue]
}

fn game(game: &str) -> Game {
    let handfuls = &game[1 + game.find(':').expect("every line should contain a colon")..];
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

pub fn first(input: String) -> String {
    (1..)
        .zip(games(&input))
        .filter(|(_, game)| is_game_possible(game))
        .map(|(game_number, _)| game_number)
        .sum::<usize>()
        .to_string()
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

pub fn second(input: String) -> String {
    games(&input)
        .map(maximum_counts)
        .map(power)
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
        test_on_input(DAY, Puzzle::First, Input::Real, 2149);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 2286);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::Real, 71274);
    }
}
