use std::collections::HashSet;

use itertools::Itertools;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

use crate::flow_network::maximum_flow;

fn network(input: &str) -> (Vec<&str>, Vec<(&str, isize, &str)>) {
    let mut components = HashSet::new();
    let mut connections = vec![];
    for line in input.lines() {
        let (from, tos) = line
            .split_once(": ")
            .expect("every line should contain a colon succeeded by a space");
        components.insert(from);
        for to in tos.split(' ') {
            components.insert(to);
            connections.push((from, 1, to));
            connections.push((to, 1, from));
        }
    }
    (components.into_iter().collect_vec(), connections)
}

fn separate_groups<'input>(
    components: &mut [&'input str],
    connections: &[(&'input str, isize, &'input str)],
    number_of_wires_to_cut: isize,
) -> (HashSet<&'input str>, HashSet<&'input str>) {
    let mut rng = SmallRng::from_seed([0; 32]);
    loop {
        components.shuffle(&mut rng);
        let (source, sink) = (components[0], components[1]);
        let (maximum_flow, cut) = maximum_flow(
            components.iter().copied(),
            connections.iter().copied(),
            source,
            sink,
        );
        if maximum_flow == number_of_wires_to_cut {
            return cut;
        }
    }
}

pub fn first(input: &str) -> String {
    let (mut components, connections) = network(input);
    let groups = separate_groups(&mut components, &connections, 3);
    (groups.0.len() * groups.1.len()).to_string()
}

pub fn second(_input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, Input, Puzzle};

    const DAY: usize = 25;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 54);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 562_912);
    }
}
