use std::collections::HashSet;

type Network<'input> = (HashSet<&'input str>, Vec<(&'input str, &'input str)>);

#[allow(unused)]
fn network(input: &str) -> Network {
    let mut components = HashSet::new();
    let mut connections = vec![];
    for line in input.lines() {
        let (from, tos) = line
            .split_once(": ")
            .expect("every line should contain a colon succeeded by a space");
        components.insert(from);
        for to in tos.split(' ') {
            components.insert(to);
            connections.push((from, to));
        }
    }
    (components, connections)
}

pub fn first(_input: &str) -> String {
    todo!();
}

pub fn second(_input: &str) -> String {
    todo!();
}

// #[cfg(test)]
// mod tests {
//     use crate::{tests::*, InputType, Puzzle};

//     const DAY: usize = 25;

//     #[test]
//     fn first_example() {
//         test_on_input(DAY, Puzzle::First, InputType::Example(0), 54);
//     }
// }
