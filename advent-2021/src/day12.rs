use std::collections::{BTreeMap, BTreeSet};

type CaveSystem<'input> = BTreeMap<&'input str, Vec<&'input str>>;

pub fn first(input: &str) -> String {
    let cave_system = parse_cave_system(input);
    num_paths(&cave_system, false).to_string()
}

pub fn second(input: &str) -> String {
    let cave_system = parse_cave_system(input);
    num_paths(&cave_system, true).to_string()
}

fn parse_connection<'input>(cave_system: &mut CaveSystem<'input>, line: &'input str) {
    let (left, right) = line
        .split_once('-')
        .expect("every line should be split by a hyphen");
    cave_system.entry(left).or_default().push(right);
    cave_system.entry(right).or_default().push(left);
}

fn parse_cave_system(input: &str) -> CaveSystem {
    let mut cave_system = CaveSystem::new();
    for line in input.lines() {
        parse_connection(&mut cave_system, line);
    }
    cave_system
        .iter_mut()
        .for_each(|(_, adjacencies)| adjacencies.sort());
    cave_system
}

fn is_small_cave(cave_name: &str) -> bool {
    cave_name
        .chars()
        .next()
        .expect("cave name should not be empty")
        .is_ascii_lowercase()
}

fn num_paths_to_end<'input>(
    cave_system: &CaveSystem<'input>,
    visited: &mut BTreeSet<&'input str>,
    free_small_cave_visit: bool,
    current: &'input str,
) -> usize {
    if current == "end" {
        return 1;
    }

    if is_small_cave(current) {
        visited.insert(current);
    }
    let mut num_paths = 0;
    for &adjacent in cave_system[current].iter() {
        if !visited.contains(adjacent) {
            num_paths += num_paths_to_end(
                cave_system,
                &mut visited.clone(),
                free_small_cave_visit,
                adjacent,
            );
        } else if free_small_cave_visit && adjacent != "start" {
            num_paths += num_paths_to_end(cave_system, visited, false, adjacent);
        }
    }
    num_paths
}

fn num_paths(cave_system: &CaveSystem, visit_small_cave_twice: bool) -> usize {
    let mut visited = BTreeSet::new();
    num_paths_to_end(cave_system, &mut visited, visit_small_cave_twice, "start")
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 12;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 226);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 4912);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 3509);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 150004);
    }
}
