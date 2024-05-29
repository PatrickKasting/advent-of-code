use std::{cmp, iter};

use itertools::Itertools;

use crate::{string::usizes, HashMap};

type Spring = u8;
type GroupSize = usize;

pub fn first(input: &str) -> String {
    sum_of_number_of_arrangements(input, 1)
}

pub fn second(input: &str) -> String {
    sum_of_number_of_arrangements(input, 5)
}

fn sum_of_number_of_arrangements(input: &str, number_of_unfold_copies: usize) -> String {
    input
        .lines()
        .map(|line| number_of_arrangements(line, number_of_unfold_copies))
        .sum::<usize>()
        .to_string()
}

fn number_of_arrangements(line: &str, number_of_unfold_copies: usize) -> usize {
    let (row, group_sizes) = row_and_group_sizes(line, number_of_unfold_copies);
    arrangements(&mut HashMap::new(), &row, &group_sizes)
}

fn arrangements(
    cache: &mut HashMap<(*const Spring, *const GroupSize), usize>,
    row: &[Spring],
    group_sizes: &[GroupSize],
) -> usize {
    if let Some(&cached) = cache.get(&(row.as_ptr(), group_sizes.as_ptr())) {
        return cached;
    }

    if row.is_empty() {
        return group_sizes.is_empty().into();
    }
    if group_sizes.is_empty() {
        return (!row.contains(&b'#')).into();
    }

    let first_is_operational =
        (row[0] != b'#').then(|| arrangements(cache, &row[1..], group_sizes));
    let first_is_damaged = prefix_is_possible_group(row, group_sizes[0]).then(|| {
        let suffix_start = cmp::min(group_sizes[0] + 1, row.len());
        arrangements(cache, &row[suffix_start..], &group_sizes[1..])
    });

    let arrangements = first_is_operational.unwrap_or(0) + first_is_damaged.unwrap_or(0);
    cache.insert((row.as_ptr(), group_sizes.as_ptr()), arrangements);
    arrangements
}

fn prefix_is_possible_group(row: &[Spring], size: GroupSize) -> bool {
    let Some(group_maybe) = row.get(..size) else {
        return false;
    };
    let separated = row.get(size) != Some(&b'#');
    let all_damaged = !group_maybe.iter().any(|&spring| spring == b'.');
    separated && all_damaged
}

fn row_and_group_sizes(
    line: &str,
    number_of_unfold_copies: usize,
) -> (Vec<Spring>, Vec<GroupSize>) {
    let (row, group_sizes) = line
        .split_once(' ')
        .expect("a space should separate row and group sizes");

    let repeated_row = iter::repeat(row.as_bytes().iter()).take(number_of_unfold_copies);
    let interspersed_unknowns = Itertools::intersperse(repeated_row, [b'?'].iter());
    let row = interspersed_unknowns.flatten().copied().collect_vec();

    let group_sizes = usizes(group_sizes).repeat(number_of_unfold_copies);

    (row, group_sizes)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::{
        super::tests::{test_on_input, YEAR},
        *,
    };
    use crate::{input, tests::*, Input, Puzzle};

    use super::number_of_arrangements;

    const DAY: usize = 12;

    #[test]
    fn first_example() {
        let input = input(YEAR, DAY, Input::Example(0));
        test_cases(
            |line| number_of_arrangements(line, 1),
            input.lines().zip_eq([1, 4, 1, 1, 4, 10]),
        );
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 7694);
    }

    #[test]
    fn second_example() {
        let input = input(YEAR, DAY, Input::Example(0));
        test_cases(
            |line| number_of_arrangements(line, 5),
            input.lines().zip_eq([1, 16384, 1, 16, 2500, 506_250]),
        );
    }

    #[test]
    fn second_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            5_071_883_216_318_usize,
        );
    }

    #[test]
    fn number_of_arrangements_empty_row() {
        let cases = [(" ", 1), (" 1", 0)];
        test_cases(|line| super::number_of_arrangements(line, 1), cases);
    }

    #[test]
    fn number_of_arrangements_empty_group_sizes() {
        let cases = [(".??.? ", 1), ("# ", 0)];
        test_cases(|line| super::number_of_arrangements(line, 1), cases);
    }

    #[test]
    fn number_of_arrangements_one_group() {
        let cases = [(".??.? 1", 3), ("..# 1", 1)];
        test_cases(|line| super::number_of_arrangements(line, 1), cases);
    }

    #[test]
    fn row_and_group_sizes_unfold() {
        let function = |line| row_and_group_sizes(line, 5);
        let cases = [
            (".# 1", (b".#?.#?.#?.#?.#".into(), vec![1, 1, 1, 1, 1])),
            (
                "???.### 1,1,3",
                (
                    b"???.###????.###????.###????.###????.###".into(),
                    vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3],
                ),
            ),
        ];
        test_cases(function, cases);
    }
}
