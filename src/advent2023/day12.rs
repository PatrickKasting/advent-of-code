use itertools::Itertools;

use crate::parsing::parse;

fn row_and_group_sizes(line: &str) -> (&[u8], Vec<usize>) {
    let (row, group_sizes) = line
        .split_once(' ')
        .expect("a space should separate row and group sizes");
    let group_sizes = group_sizes.split(',').map(parse).collect_vec();
    (row.as_bytes(), group_sizes)
}

fn is_damaged(row: &[u8], position: usize) -> bool {
    row.get(position) == Some(&b'#')
}

fn valid_positions(row: &[u8], group_size: usize) -> impl Iterator<Item = usize> + '_ {
    row.windows(group_size)
        .enumerate()
        .take_while(|(position, _)| !is_damaged(row, position.wrapping_sub(1)))
        .filter(move |(position, window)| {
            window.iter().all(|spring| [b'#', b'?'].contains(spring))
                && !is_damaged(row, position + group_size)
        })
        .map(|(position, _)| position)
}

fn arrangements(row: &[u8], group_sizes: &[usize]) -> usize {
    let Some((group_size, remaining_group_sizes)) = group_sizes.split_first() else {
        return usize::from(!row.contains(&b'#'));
    };
    valid_positions(row, *group_size)
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

fn number_of_arrangements(line: &str) -> usize {
    let (row, group_sizes) = row_and_group_sizes(line);
    arrangements(row, &group_sizes)
}

fn sum_of_number_of_arrangements(input: &str) -> usize {
    input.lines().map(number_of_arrangements).sum()
}

pub fn first(input: &str) -> String {
    sum_of_number_of_arrangements(input).to_string()
}

pub fn second(_input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::super::tests::{test_on_input, YEAR};
    use crate::{input, tests::*, Input, Puzzle};

    use super::number_of_arrangements;

    const DAY: usize = 12;

    #[test]
    fn first_example() {
        let input = input(YEAR, DAY, Input::Example(0));
        test_cases(number_of_arrangements, input.lines(), [1, 4, 1, 1, 4, 10]);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 7694);
    }

    // #[test]
    // fn second_example() {
    //     let input = input(DAY, Input::Example(0));
    //     test_cases(
    //         |line| number_of_arrangements(line),
    //         input.lines(),
    //         [1, 16384, 1, 16, 2500, 506250],
    //     );
    // }
}
