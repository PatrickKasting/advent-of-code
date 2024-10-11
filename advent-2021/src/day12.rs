use ahash::{AHashMap, AHashSet};

type CaveSystem<'input> = AHashMap<Cave<'input>, Vec<Cave<'input>>>;
type Cave<'input> = &'input [u8];

pub fn first(input: &str) -> String {
    let cave_system = cave_system(input);
    number_of_paths(&cave_system, b"start", AHashSet::new(), true).to_string()
}

pub fn second(input: &str) -> String {
    let cave_system = cave_system(input);
    number_of_paths(&cave_system, b"start", AHashSet::new(), false).to_string()
}

fn number_of_paths<'input>(
    cave_system: &CaveSystem<'input>,
    position: Cave<'input>,
    mut visited: AHashSet<Cave<'input>>,
    small_cave_twice: bool,
) -> usize {
    if position == b"end" {
        return 1;
    }

    if is_small_cave(position) {
        visited.insert(position);
    }

    let mut number_of_paths = 0;
    for &cave in &cave_system[position] {
        if !visited.contains(cave) {
            number_of_paths +=
                self::number_of_paths(cave_system, cave, visited.clone(), small_cave_twice);
        } else if !small_cave_twice && cave != b"start" {
            number_of_paths += self::number_of_paths(cave_system, cave, visited.clone(), true);
        }
    }
    number_of_paths
}

fn is_small_cave(cave: Cave) -> bool {
    cave[0].is_ascii_lowercase()
}

fn cave_system(input: &str) -> CaveSystem {
    let mut cave_system = CaveSystem::new();
    for line in input.lines() {
        let (left, right) = line
            .split_once('-')
            .expect("line should contain two caves separated by a hyphen");
        let (left, right) = (left.as_bytes(), right.as_bytes());
        cave_system.entry(left).or_default().push(right);
        cave_system.entry(right).or_default().push(left);
    }
    cave_system
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};

    use crate::tests::{input, test_on_input};

    use super::*;

    const DAY: usize = 12;

    #[test]
    fn first_examples() {
        let function = |example| {
            let input = input(DAY, Input::Example(example));
            let cave_system = cave_system(&input);
            number_of_paths(&cave_system, b"start", AHashSet::new(), true)
        };
        let cases = [(0, 10), (1, 19), (2, 226)];
        test::cases(function, cases);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 4912);
    }

    #[test]
    fn second_example() {
        let function = |example| {
            let input = input(DAY, Input::Example(example));
            let cave_system = cave_system(&input);
            number_of_paths(&cave_system, b"start", AHashSet::new(), false)
        };
        let cases = [(0, 36), (1, 103), (2, 3509)];
        test::cases(function, cases);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 150_004);
    }
}
