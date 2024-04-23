use std::collections::HashSet;

use crate::data_structures::grid::{Direction, Position};

type Motion = (Direction, usize);

pub fn first(input: &str) -> String {
    tail_positions::<2>(motions(input)).len().to_string()
}

pub fn second(input: &str) -> String {
    tail_positions::<10>(motions(input)).len().to_string()
}

fn tail_positions<const NUMBER_OF_KNOTS: usize>(
    motions: impl Iterator<Item = Motion>,
) -> HashSet<Position> {
    let initial_position = Position::new(0, 0);
    let mut rope = [initial_position; NUMBER_OF_KNOTS];
    let mut tail_positions = HashSet::from([initial_position]);
    for (direction, number_of_steps) in motions {
        for _ in 0..number_of_steps {
            step(&mut rope, direction);
            tail_positions.insert(*rope.last().expect("rope should have at least one knot"));
        }
    }
    tail_positions
}

fn step(rope: &mut [Position], direction: Direction) {
    rope[0] = rope[0].neighbor(direction);
    for knot_index in 1..rope.len() {
        let [knot, puller] = [rope[knot_index], rope[knot_index - 1]];
        rope[knot_index] = knot_position(knot, puller);
    }
}

fn knot_position(knot: Position, puller: Position) -> Position {
    let [mut row, mut column] = [knot.row(), knot.column()];
    let [row_difference, column_difference] =
        [Position::row, Position::column].map(|coordinate| coordinate(puller) - coordinate(knot));
    if row_difference.abs() > 1 {
        row += row_difference / 2;
        column += column_difference.signum();
    } else if column_difference.abs() > 1 {
        column += column_difference / 2;
        row += row_difference.signum();
    }
    Position::new(row, column)
}

fn motions(input: &str) -> impl Iterator<Item = Motion> + '_ {
    input.lines().map(|line| {
        let direction = Direction::from_up_down_left_or_right(line.as_bytes()[0] as char);
        let number_of_steps = line[2..]
            .parse()
            .expect("number of steps should be numerical");
        (direction, number_of_steps)
    })
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 9;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 13);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 6087);
    }

    #[test]
    fn second_examples() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 1);
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 36);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 2493);
    }
}
