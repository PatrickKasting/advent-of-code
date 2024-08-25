use regex::Regex;

type Player = usize;
type Position = u16;
type Score = i16;
type Result = usize;

const STARTING_POSITION: &str = "input should include starting positions";

const NUM_POSITIONS: Position = 10;
const NUM_PLAYERS: Player = 2;
const NUM_ROLLS_PER_TURN: usize = 3;
const MAX_DETERMINISTIC_DIE_FACE: Position = 100;
const DETERMINISTIC_GOAL: Score = 1000;
const QUANTUM_GOAL: Score = 21;

fn parse_starting_positions(input: &str) -> [Position; 2] {
    let regex = Regex::new(r"\d+").expect("regex should be valid");
    let mut lines = input.lines();
    let mut next_position = || {
        regex
            .find_iter(
                lines
                    .next()
                    .expect("input should contain exactly two lines"),
            )
            .nth(1)
            .expect(STARTING_POSITION)
            .as_str()
            .parse()
            .expect(STARTING_POSITION)
    };
    [next_position(), next_position()].map(|position: Position| position - 1)
}

fn next_player(current_player: Player) -> Player {
    (current_player + 1) % NUM_PLAYERS
}

fn destination(current_position: Position, num_steps: Position) -> Position {
    (current_position + num_steps) % NUM_POSITIONS
}

fn score(destination: Position) -> Score {
    destination as Score + 1
}

fn deterministic_game(mut positions: [Position; 2]) -> Result {
    let mut die = (1..=MAX_DETERMINISTIC_DIE_FACE).cycle();
    let mut num_die_rolls = 0;
    let mut roll_thrice = || {
        num_die_rolls += NUM_ROLLS_PER_TURN;
        (0..NUM_ROLLS_PER_TURN)
            .map(|_| die.next().expect("the die should never run out"))
            .sum()
    };

    let mut scores = [0, 0];
    let mut current_player = 0;
    loop {
        positions[current_player] = destination(positions[current_player], roll_thrice());
        scores[current_player] += score(positions[current_player]);
        if scores[current_player] >= DETERMINISTIC_GOAL {
            return scores[next_player(current_player)] as Result * num_die_rolls;
        }
        current_player = next_player(current_player);
    }
}

pub fn first(input: &str) -> String {
    let starting_positions = parse_starting_positions(input);
    deterministic_game(starting_positions).to_string()
}

const NUM_QUANTUM_DIE_COMBINATIONS: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];

fn quantum_win_counts(
    [current_player_score, other_player_score]: [Score; 2],
    [current_player_position, other_player_position]: [Position; 2],
) -> [usize; 2] {
    debug_assert!(current_player_score < QUANTUM_GOAL);
    if other_player_score >= QUANTUM_GOAL {
        return [0, 1];
    }

    let mut before_move_counts = [0, 0];
    for (sum, num_combinations) in NUM_QUANTUM_DIE_COMBINATIONS
        .iter()
        .enumerate()
        .map(|(sum, &num_combinations)| ((sum + 3) as Position, num_combinations))
    {
        let current_player_position = destination(current_player_position, sum);
        let current_player_score = current_player_score + score(current_player_position);
        let after_move_counts = quantum_win_counts(
            [other_player_score, current_player_score],
            [other_player_position, current_player_position],
        );
        before_move_counts[0] += after_move_counts[1] * num_combinations;
        before_move_counts[1] += after_move_counts[0] * num_combinations;
    }
    before_move_counts
}

fn quantum_game(positions: [Position; 2]) -> usize {
    quantum_win_counts([0, 0], positions)
        .into_iter()
        .max()
        .expect("two win counts should always be computed")
}

pub fn second(input: &str) -> String {
    let starting_positions = parse_starting_positions(input);
    quantum_game(starting_positions).to_string()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use super::*;
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
    fn starting_positions() {
        assert_starting_positions(Input::Example(0), 3, 7);
        assert_starting_positions(Input::PuzzleInput, 4, 5);
    }

    #[test]
    fn quantum_game_max_zero_moves() {
        for first_player_score in 0..QUANTUM_GOAL {
            for first_player_position in 0..NUM_POSITIONS {
                for second_player_position in 0..NUM_POSITIONS {
                    let win_counts = quantum_win_counts(
                        [first_player_score, QUANTUM_GOAL],
                        [first_player_position, second_player_position],
                    );
                    assert_eq!(win_counts, [0, 1]);
                }
            }
        }
    }

    #[test]
    fn quantum_game_max_one_move() {
        for second_player_score in 0..QUANTUM_GOAL {
            for first_player_position in 0..NUM_POSITIONS {
                for second_player_position in 0..NUM_POSITIONS {
                    let win_counts = quantum_win_counts(
                        [QUANTUM_GOAL - 1, second_player_score],
                        [first_player_position, second_player_position],
                    );
                    assert_eq!(win_counts, [27, 0]);
                }
            }
        }
    }

    #[test]
    fn quantum_game_max_two_moves() {
        for first_player_position in [3, 5] {
            for second_player_position in 0..NUM_POSITIONS {
                let win_counts = quantum_win_counts(
                    [QUANTUM_GOAL - 2, QUANTUM_GOAL - 1],
                    [first_player_position, second_player_position],
                );
                assert_eq!(win_counts, [21, 6 * 27])
            }
        }
    }

    #[test]
    fn quantum_game_max_three_moves() {
        for first_player_position in [3, 5] {
            for second_player_position in [1, 7] {
                let win_counts = quantum_win_counts(
                    [QUANTUM_GOAL - 2, QUANTUM_GOAL - 2],
                    [first_player_position, second_player_position],
                );
                assert_eq!(win_counts, [21 + 6 * 27, 6 * 26])
            }
        }
    }

    fn assert_starting_positions(input: Input, player1: Position, player2: Position) {
        let starting_positions = parse_starting_positions(&self::input(DAY, input));
        assert_eq!(starting_positions, [player1, player2]);
    }
}
