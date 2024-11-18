use std::{char, collections::VecDeque};

use easy_cast::Cast;
use itertools::Itertools;

type Cups = VecDeque<Cup>;
type Cup = usize;

pub fn first_answer(input: &str) -> String {
    let mut cups = cups(input);
    game(&mut cups, 100);
    labels(&mut cups)
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn game(cups: &mut Cups, number_of_moves: usize) {
    for _ in 0..number_of_moves {
        move_cups(cups);
    }
}

fn move_cups(cups: &mut Cups) {
    let number_of_cups = cups.len();
    let picked_up = cups.drain(1..=3).collect_vec();
    for destination in (cups[0]..=number_of_cups).chain(1..cups[0]).rev() {
        if let Some(destination) = cups.iter().position(|&cup| cup == destination) {
            for cup in picked_up.into_iter().rev() {
                cups.insert(destination + 1, cup);
            }
            break;
        }
    }
    cups.rotate_left(1);
}

fn labels(cups: &mut Cups) -> String {
    let one = cups
        .iter()
        .position(|&cup| cup == 1)
        .expect("cups should be labeled from one and up");
    cups.rotate_left(one);
    cups.range(1..)
        .map(|&cup| char::from_digit(cup.cast(), 10).expect("cup labels should be single digit"))
        .collect()
}

fn cups(input: &str) -> Cups {
    input
        .chars()
        .filter(|char| char.is_numeric())
        .map(|char| char as Cup - '0' as Cup)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use infrastructure::{Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 23;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), "67384529");
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, "46978532");
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 291);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 32519);
    }

    #[test]
    fn moves() {
        let mut cups = cups(&input(DAY, Input::Example(0)));
        let mut actual = vec![];
        for _ in 0..10 {
            move_cups(&mut cups);
            actual.push(cups.clone());
        }
        let expected = [
            [2, 8, 9, 1, 5, 4, 6, 7, 3],
            [5, 4, 6, 7, 8, 9, 1, 3, 2],
            [8, 9, 1, 3, 4, 6, 7, 2, 5],
            [4, 6, 7, 9, 1, 3, 2, 5, 8],
            [1, 3, 6, 7, 9, 2, 5, 8, 4],
            [9, 3, 6, 7, 2, 5, 8, 4, 1],
            [2, 5, 8, 3, 6, 7, 4, 1, 9],
            [6, 7, 4, 1, 5, 8, 3, 9, 2],
            [5, 7, 4, 1, 8, 3, 9, 2, 6],
            [8, 3, 7, 4, 1, 9, 2, 6, 5],
        ]
        .map(VecDeque::from);
        for (number_of_moves, (actual, expected)) in (1..).zip(actual.into_iter().zip_eq(expected))
        {
            assert_eq!(
                actual, expected,
                "actual should equal expected after {number_of_moves} moves"
            );
        }
    }
}
