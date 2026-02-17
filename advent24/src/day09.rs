use itertools::Itertools;

type DiskMap = Vec<usize>;

pub fn first_answer(input: &str) -> String {
    checksum_blocks(disk_map(input)).to_string()
}

pub fn second_answer(input: &str) -> String {
    checksum_files(disk_map(input)).to_string()
}

fn checksum_blocks(mut disk_map: DiskMap) -> usize {
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

fn checksum_files(mut disk_map: DiskMap) -> usize {
    let mut position = 0;
    let mut moved = vec![false; disk_map.len()];
    let mut checksum = 0;
    for front in 0..disk_map.len() {
        if front % 2 == 0 && !moved[front] {
            let id = front / 2;
            checksum += id * sum(position, disk_map[front]);
        } else {
            let mut back = disk_map.len() - 1;
            while front < back && disk_map[front] != 0 {
                if !moved[back] && disk_map[back] <= disk_map[front] {
                    disk_map[front] -= disk_map[back];
                    let id = back / 2;
                    checksum += id * sum(position, disk_map[back]);
                    position += disk_map[back];
                    moved[back] = true;
                }
                back -= 2;
            }
        }
        position += disk_map[front];
    }
    checksum
}

fn sum(start: usize, len: usize) -> usize {
    (start..start + len).sum()
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
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 2858);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            6_381_624_803_796_usize,
        );
    }

    #[test]
    fn single() {
        assert_checksum_blocks("3", "000");
    }

    #[test]
    fn zero_first_and_last() {
        assert_checksum_blocks("03240", "11");
    }

    #[test]
    fn small() {
        assert_checksum_blocks("12345", "022111222");
    }

    fn assert_checksum_blocks(input: &str, compacted: &str) {
        let actual = checksum_blocks(disk_map(input));
        let expected: usize = compacted
            .chars()
            .enumerate()
            .map(|(position, id)| position * (id as usize - '0' as usize))
            .sum();
        assert_eq!(actual, expected, "block checksum should be expected");
    }
}
