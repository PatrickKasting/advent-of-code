use std::collections::HashMap;

use itertools::Itertools;

use crate::math::least_common_multiple;

type Connection<'c> = (&'c str, (&'c str, &'c str));
type Network<'n> = HashMap<&'n str, (&'n str, &'n str)>;

fn connection(connection: &str) -> Connection {
    (&connection[0..3], (&connection[7..10], &connection[12..15]))
}

fn network(connections: &str) -> Network {
    connections.lines().map(connection).collect()
}

fn directions_and_network(input: &str) -> (&str, Network) {
    let (directions, network) = input
        .split_once("\n\n")
        .expect("input should contain directions and map");
    (directions, self::network(network))
}

fn ghost_starting_nodes<'network, 'input>(network: &'network Network<'input>) -> Vec<&'input str> {
    network
        .keys()
        .filter(|node| node.ends_with('A'))
        .copied()
        .collect_vec()
}

fn step<'network>(network: &'network Network, from: &str, direction: char) -> &'network str {
    match direction {
        'L' => network[from].0,
        'R' => network[from].1,
        _ => unreachable!("direction should be either 'L' or 'R'"),
    }
}

fn cycle_length(
    network: &Network,
    directions: &str,
    source: &str,
    is_destination: &impl Fn(&str) -> bool,
) -> usize {
    let mut node = source;
    let mut cycle_length = 0;
    for direction in directions.chars().cycle() {
        if is_destination(node) {
            return cycle_length;
        }
        node = step(network, node, direction);
        cycle_length += 1;
    }
    unreachable!("loop should return because we reach destination")
}

fn number_of_steps(
    network: Network,
    directions: &str,
    sources: Vec<&str>,
    is_destination: impl Fn(&str) -> bool,
) -> usize {
    let cycle_lengths = sources
        .into_iter()
        .map(|source| cycle_length(&network, directions, source, &is_destination));
    least_common_multiple(cycle_lengths)
}

pub fn first(input: String) -> String {
    let (directions, network) = directions_and_network(&input);
    let is_destination = |node: &str| node == "ZZZ";
    number_of_steps(network, directions, vec!["AAA"], is_destination).to_string()
}

pub fn second(input: String) -> String {
    let (directions, network) = directions_and_network(&input);
    let starting_nodes = ghost_starting_nodes(&network);
    let is_destination = |node: &str| node.ends_with('Z');
    number_of_steps(network, directions, starting_nodes, is_destination).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, Input, Puzzle};

    const DAY: usize = 8;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 2);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 6);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::Real, 18673);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(2), 6);
    }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::Real, 251515496);
    // }
}
