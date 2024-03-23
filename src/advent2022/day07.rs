use std::{collections::HashMap, iter};

use itertools::Itertools;

use crate::strings::parse;

type Size = usize;

struct Directory<'input> {
    children: HashMap<&'input str, Directory<'input>>,
    file_size_sum: Size,
}

pub fn first(input: &str) -> String {
    let total_sizes = total_sizes(&root(input));
    total_sizes
        .into_iter()
        .filter(|&size| size < 100_000)
        .sum::<Size>()
        .to_string()
}

pub fn second(input: &str) -> String {
    let mut total_sizes = total_sizes(&root(input));
    let space_to_be_freed = total_sizes[0] - (70_000_000 - 30_000_000);
    total_sizes.sort_unstable();
    total_sizes
        .into_iter()
        .find(|&size| size >= space_to_be_freed)
        .expect("at least one directory should free enough space")
        .to_string()
}

fn total_sizes(directory: &Directory) -> Vec<Size> {
    let child_total_sizes = directory.children.values().map(total_sizes).collect_vec();
    let this_total_size = directory.file_size_sum
        + child_total_sizes
            .iter()
            .map(|total_sizes| total_sizes[0])
            .sum::<Size>();
    iter::once(this_total_size)
        .chain(child_total_sizes.into_iter().flatten())
        .collect_vec()
}

fn root(input: &str) -> Directory {
    let mut current_path = Vec::new();
    let mut root = empty_directory();
    let mut current_directory = &mut root;

    for line in input.lines() {
        match &line[0..4] {
            "$ ls" | "dir " => (),
            "$ cd" => {
                change_path(&mut current_path, &line[5..]);
                current_directory = directory(&mut root, &current_path);
            }
            _ => {
                let (size, _) = line
                    .split_once(' ')
                    .expect("size and name should be separated by a space");
                current_directory.file_size_sum += parse::<&str, Size>(size);
            }
        }
    }
    root
}

fn change_path<'input>(path: &mut Vec<&'input str>, destination: &'input str) {
    match destination {
        "/" => path.clear(),
        ".." => {
            path.pop();
        }
        _ => path.push(destination),
    }
}

fn directory<'input, 'root>(
    root: &'root mut Directory<'input>,
    path: &[&'input str],
) -> &'root mut Directory<'input> {
    let mut directory = root;
    for &directory_name in path {
        directory = directory
            .children
            .entry(directory_name)
            .or_insert_with(empty_directory);
    }
    directory
}

fn empty_directory<'input>() -> Directory<'input> {
    Directory {
        children: HashMap::new(),
        file_size_sum: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 7;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 95437);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1444896);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 24933642);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 404395);
    }
}
