use std::collections::HashMap;

use itertools::Itertools;

use crate::math::least_common_multiple;

type Node<'input> = &'input str;
type Connection<'input> = (Node<'input>, (Node<'input>, Node<'input>));
type Network<'input> = HashMap<Node<'input>, (Node<'input>, Node<'input>)>;

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

fn step<'input>(network: &Network<'input>, from: Node, direction: char) -> Node<'input> {
    match direction {
        'L' => network[from].0,
        'R' => network[from].1,
        _ => unreachable!("direction should be either 'L' or 'R'"),
    }
}

fn number_of_steps<'input>(
    network: &Network<'input>,
    directions: &str,
    mut node: Node<'input>,
) -> usize {
    for (time, direction) in directions.chars().cycle().enumerate() {
        if node.ends_with('Z') {
            return time;
        }
        node = step(network, node, direction);
    }
    unreachable!("directions should repeat indefinitely")
}

fn ghost_starting_nodes<'network, 'input>(
    network: &'network Network<'input>,
) -> impl Iterator<Item = Node<'input>> + 'network {
    network.keys().copied().filter(|node| node.ends_with('A'))
}

pub fn first(input: String) -> String {
    let (directions, network) = directions_and_network(&input);
    number_of_steps(&network, directions, "AAA").to_string()
}

pub fn second(input: String) -> String {
    // We assume that for each ghost, the cycle length is equal to the distance from the starting
    // node to the destination node.
    let (directions, network) = directions_and_network(&input);
    let cycle_lengths = ghost_starting_nodes(&network)
        .map(|node| number_of_steps(&network, directions, node))
        .collect_vec();
    least_common_multiple(cycle_lengths).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, Input, Puzzle};

    const DAY: usize = 8;

    #[test]
    fn first_examples() {
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

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::Real, 17972669116327usize);
    }
}
