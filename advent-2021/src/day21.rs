use std::array;

use ahash::AHashMap;
use itertools::Itertools;
use shared::{string::usizes, vector::Vector};

type Space = Number;
type Score = Number;
type Player = usize;
type Roll = Number;
type Number = usize;

type State = ([Space; 2], [Score; 2], Player);

const TRACK_SIZE: Roll = 10;
const DETERMINISTIC_SCORE_TARGET: Score = 1000;
const QUANTUM_SCORE_TARGET: Score = 21;

pub fn first(input: &str) -> String {
    let starting_spaces = starting_spaces(input);
    let (losing_score, number_of_rolls) = deterministic_game(starting_spaces);
    (losing_score * number_of_rolls).to_string()
}

pub fn second(input: &str) -> String {
    let starting_spaces = starting_spaces(input);
    let number_of_wins = quantum_game(&mut AHashMap::new(), starting_spaces, [0, 0], 0);
    number_of_wins
        .into_iter()
        .max()
        .expect("two players should play")
        .to_string()
}

fn deterministic_game(mut positions: [Space; 2]) -> (Score, usize) {
    let mut die = (1..=100).cycle();
    let mut number_of_rolls = 0;
    let mut roll = || {
        number_of_rolls += 1;
        die.next().expect("die iterator should never deplete")
    };

    let mut scores = [0, 0];
    for player in (0..=1).cycle() {
        let number_of_spaces = roll() + roll() + roll();
        mov(&mut positions, &mut scores, player, number_of_spaces);
        if scores[player] >= DETERMINISTIC_SCORE_TARGET {
            return (scores[player ^ 1], number_of_rolls);
        }
    }
    unreachable!("game should continue until one player has enough points");
}

fn quantum_game(
    cache: &mut AHashMap<State, [usize; 2]>,
    positions: [Space; 2],
    scores: [Score; 2],
    player: Player,
) -> [usize; 2] {
    if scores[player ^ 1] >= QUANTUM_SCORE_TARGET {
        return array::from_fn(|index| (index == player ^ 1).into());
    }

    let state = (positions, scores, player);
    if let Some(&cached) = cache.get(&state) {
        return cached;
    }

    let mut number_of_wins = [0, 0];
    let rolls = (3..=9).zip_eq([1, 3, 6, 7, 6, 3, 1]);
    for (sum, frequency) in rolls {
        let (mut positions, mut scores) = (positions, scores);
        mov(&mut positions, &mut scores, player, sum);
        let number_of_wins_new_state =
            quantum_game(cache, positions, scores, player ^ 1).mul(frequency);
        number_of_wins = number_of_wins.add(number_of_wins_new_state);
    }

    let old_cached = cache.insert(state, number_of_wins);
    debug_assert!(
        old_cached.is_none(),
        "state should not already exist in the cache"
    );
    number_of_wins
}

fn mov(
    positions: &mut [usize; 2],
    scores: &mut [Score; 2],
    player: usize,
    number_of_spaces: usize,
) {
    positions[player] += number_of_spaces;
    positions[player] %= TRACK_SIZE;
    scores[player] += positions[player] + 1;
}

fn starting_spaces(input: &str) -> [Space; 2] {
    let mut lines = input.lines();
    array::from_fn(|_| usizes(lines.next().expect("input should have two lines"))[1] - 1)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::{input, test_on_input};

    const DAY: usize = 21;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 739785);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1002474);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 444356092776315usize);
    }

    #[test]
    fn second_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            919758187195363usize,
        );
    }

    #[test]
    fn starting_spaces() {
        let actual = super::starting_spaces(&input(DAY, Input::Example(0)));
        let expected = [3, 7];
        assert_eq!(actual, expected);
    }
}
