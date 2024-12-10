use std::cmp::Ordering;

use itertools::Itertools;

type DiskMap = Vec<usize>;

pub fn first_answer(input: &str) -> String {
    checksum(disk_map(input)).to_string()
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn checksum(mut disk_map: DiskMap) -> usize {
    let (mut front, mut back) = (0, disk_map.len() - 1);
    let mut checksum = 0;
    for position in 0.. {
        while disk_map[front] == 0 {
            front += 1;
            if back < front {
                return checksum;
            }
        }
        while disk_map[back] == 0 {
            back -= 2;
            if back < front {
                return checksum;
            }
        }
        if front % 2 == 0 {
            let id = front / 2;
            checksum += position * id;
        } else {
            let id = back / 2;
            checksum += position * id;
            disk_map[back] -= 1;
        }
        disk_map[front] -= 1;
    }
    unreachable!("loop should break when indices meet");
}

fn disk_map(input: &str) -> DiskMap {
    let input = input.trim_end();
    assert!(input.len() % 2 == 1, "disk map should have odd length");
    input
        .chars()
        .map(|char| char as usize - '0' as usize)
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use super::*;
    use crate::tests::test_on_input;

    const DAY: usize = 9;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 1928);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(
            DAY,
            Puzzle::First,
            Input::PuzzleInput,
            6_359_213_660_505_usize,
        );
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 34);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1200);
    }

    #[test]
    fn single() {
        assert_correct("3", "000");
    }

    #[test]
    fn zero_first_and_last() {
        assert_correct("03240", "11");
    }

    #[test]
    fn small() {
        assert_correct("12345", "022111222");
    }

    fn assert_correct(input: &str, compacted: &str) {
        let actual = checksum(disk_map(input));
        let expected: usize = compacted
            .chars()
            .enumerate()
            .map(|(position, id)| position * (id as usize - '0' as usize))
            .sum();
        assert_eq!(actual, expected);
    }
}
