use itertools::Itertools;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

use crate::{math::graphs::flow_networks::maximum_flow, HashSet};

type One = usize;

pub fn first(input: &str) -> String {
    let (components, connections) = network(input);
    let groups = separate_groups(&components, &connections, 3);
    (groups[0].len() * groups[1].len()).to_string()
}

pub fn second(_input: &str) -> String {
    unimplemented!();
}

fn separate_groups<'input>(
    components: &[&'input str],
    connections: &[(&'input str, One, &'input str)],
    number_of_wires_to_cut: usize,
) -> [HashSet<&'input str>; 2] {
    let mut rng = SmallRng::from_seed([0; 32]);
    loop {
        let mut terminals = components.choose_multiple(&mut rng, 2);
        let (maximum_flow, cut) = maximum_flow(
            components.iter().copied(),
            connections.iter().copied(),
            terminals
                .next()
                .expect("two components should be randomly chosen"),
            terminals
                .next()
                .expect("two components should be randomly chosen"),
        );
        if maximum_flow == number_of_wires_to_cut {
            return cut;
        }
    }
}

fn network(input: &str) -> (Vec<&str>, Vec<(&str, One, &str)>) {
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

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

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
