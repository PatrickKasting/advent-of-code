#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

pub fn first_answer(input: &str) -> String {
    todo!()
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn walks(input: &str) -> impl Iterator<Item = Vec<Direction>> + '_ {
    input.lines().map(walk)
}

fn walk(mut str: &str) -> Vec<Direction> {
    let mut walk = vec![];
    while !str.is_empty() {
        let (remaining, direction) = direction(str);
        str = remaining;
        walk.push(direction);
    }
    walk
}

fn direction(str: &str) -> (&str, Direction) {
    match &str[0..1] {
        "e" => (&str[1..], Direction::East),
        "w" => (&str[1..], Direction::West),
        _ => match &str[0..2] {
            "se" => (&str[2..], Direction::SouthEast),
            "sw" => (&str[2..], Direction::SouthWest),
            "nw" => (&str[2..], Direction::NorthWest),
            "ne" => (&str[2..], Direction::NorthEast),
            _ => panic!("direction should be 'e', 'se', 'sw', 'w', 'nw', or 'ne'"),
        },
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 24;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 306);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 32629);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 291);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 32519);
    }
}
