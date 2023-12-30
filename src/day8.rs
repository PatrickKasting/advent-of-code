use std::collections::HashMap;

use itertools::Itertools;

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

fn ghost_starting_nodes<'input>(network: &Network<'input>) -> Vec<Node<'input>> {
    network
        .keys()
        .filter(|node| node.ends_with('A'))
        .copied()
        .collect_vec()
}

fn number_of_steps<'input>(
    network: Network<'input>,
    directions: &str,
    mut nodes: Vec<Node<'input>>,
    is_destination: impl Fn(Node) -> bool,
) -> usize {
    for (time, direction) in directions.chars().cycle().enumerate() {
        if nodes.iter().all(|node| is_destination(node)) {
            return time;
        }
        for node in nodes.iter_mut() {
            *node = step(&network, node, direction);
        }
    }
    unreachable!()
}

pub fn first(input: String) -> String {
    let (directions, network) = directions_and_network(&input);
    number_of_steps(network, directions, vec!["AAA"], |node| node == "ZZZ").to_string()
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

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::Real, 251515496);
    // }
}
