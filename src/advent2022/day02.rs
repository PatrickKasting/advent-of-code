use crate::strings::char_at;

type Symbol = i64; // Rock = 0, Paper = 1, Scissors = 2 OR Loss = 0, Draw = 1, Win = 2
type Game = [Symbol; 2];
type Score = i64;

pub fn first(input: &str) -> String {
    total_score(games(input), score_known_shape).to_string()
}

pub fn second(input: &str) -> String {
    total_score(games(input), score_known_outcome).to_string()
}

fn total_score(games: impl Iterator<Item = Game>, score: fn(Game) -> Score) -> Score {
    games.map(score).sum()
}

fn score_known_outcome([opponent, outcome]: [Symbol; 2]) -> Score {
    let me = (outcome + opponent - 1).rem_euclid(3);
    me + 1 + outcome * 3
}

fn score_known_shape([opponent, me]: [Symbol; 2]) -> Score {
    let outcome = (me - opponent + 1).rem_euclid(3);
    me + 1 + outcome * 3
}

fn games(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.lines().map(|line| {
        [
            char_at(line, 0) as i64 - 'A' as i64,
            char_at(line, 2) as i64 - 'X' as i64,
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 2;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 15);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 13565);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 12);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 12424);
    }
}
