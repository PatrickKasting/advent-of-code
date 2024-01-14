use std::iter;

use itertools::Itertools;

use crate::utilities::number;

fn row_and_group_sizes(line: &str, unfold_factor: usize) -> (Vec<u8>, Vec<usize>) {
    let (row, group_sizes) = line
        .split_once(' ')
        .expect("a space should separate row and group sizes");
    let unfolded_row = Itertools::intersperse(
        iter::repeat(row.as_bytes().iter().copied()).take(unfold_factor),
        [b'?'].iter().copied(),
    )
    .flatten()
    .collect_vec();
    let unfolded_group_sizes = iter::repeat(group_sizes.split(',').map(number))
        .take(unfold_factor)
        .flatten()
        .collect_vec();
    (unfolded_row, unfolded_group_sizes)
}

fn is_damaged(row: &[u8], position: usize) -> bool {
    match row.get(position) {
        Some(&spring) => spring == b'#',
        None => false,
    }
}

fn valid_positions(row: &[u8], group_size: usize) -> Vec<usize> {
    let mut valid_positions = Vec::new();
    for (position, window) in row.windows(group_size).enumerate() {
        if is_damaged(row, position.wrapping_sub(1)) {
            break;
        }
        if window.iter().all(|spring| [b'#', b'?'].contains(spring))
            && !is_damaged(row, position + group_size)
        {
            valid_positions.push(position);
        }
    }
    valid_positions
}

fn arrangements(row: &[u8], group_sizes: &[usize]) -> usize {
    let Some((group_size, remaining_group_sizes)) = group_sizes.split_first() else {
        return usize::from(!row.contains(&b'#'));
    };
    valid_positions(row, *group_size)
        .into_iter()
        .map(|position| {
            let remaining_row_start = position + group_size + 1;
            let remaining_row = if remaining_row_start < row.len() {
                &row[remaining_row_start..]
            } else {
                &[]
            };
            arrangements(remaining_row, remaining_group_sizes)
        })
        .sum()
}

fn number_of_arrangements(line: &str, unfold_factor: usize) -> usize {
    let (row, group_sizes) = row_and_group_sizes(line, unfold_factor);
    arrangements(&row, &group_sizes)
}

fn sum_of_number_of_arrangements(input: &str, unfold_factor: usize) -> usize {
    input
        .lines()
        .map(|line| number_of_arrangements(line, unfold_factor))
        .sum()
}

pub fn first(input: &str) -> String {
    sum_of_number_of_arrangements(&input, 1).to_string()
}

pub fn second(input: &str) -> String {
    sum_of_number_of_arrangements(&input, 5).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{input, tests::*, Input, Puzzle};

    use super::number_of_arrangements;

    const DAY: usize = 12;

    #[test]
    fn first_example_individual_cases() {
        let input = input(DAY, Input::Example(0));
        test_cases(
            |line| number_of_arrangements(line, 1),
            input.lines(),
            [1, 4, 1, 1, 4, 10],
        );
    }

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 21);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 7694);
    }

    #[test]
    fn second_example_individual_cases() {
        let input = input(DAY, Input::Example(0));
        test_cases(
            |line| number_of_arrangements(line, 5),
            input.lines(),
            [1, 16384, 1, 16, 2500, 506250],
        );
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 525152);
    }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::Real, 55358);
    // }
}
