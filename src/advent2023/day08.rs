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
        _ => panic!("direction should be either 'L' or 'R'"),
    }
}

fn is_starting(node: Node) -> bool {
    node.ends_with('A')
}

fn is_destination(node: Node) -> bool {
    node.ends_with('Z')
}

fn destination_and_time<'input>(
    network: &Network<'input>,
    directions: &str,
    skipped_directions: usize,
    mut node: Node<'input>,
) -> (Node<'input>, usize) {
    let directions = directions.chars().cycle().skip(skipped_directions);
    for (time, direction) in (1..).zip(directions) {
        node = step(network, node, direction);
        if is_destination(node) {
            return (node, time);
        }
    }
    panic!("directions should repeat indefinitely")
}

fn time_to_all_ghosts_at_destinations(network: &Network, directions: &str) -> usize {
    let mut ghosts = network
        .keys()
        .filter(|node| is_starting(node))
        .map(|node| destination_and_time(network, directions, 0, node))
        .collect_vec();
    let mut destination_network = HashMap::new();
    while !ghosts.iter().map(|(_, time)| *time).all_equal() {
        let ghost = ghosts
            .iter_mut()
            .min_by_key(|(_, time)| *time)
            .expect("there should be at least one ghost");
        let direction_index = ghost.1 % directions.len();
        let (destination, time) = *destination_network
            .entry((ghost.0, direction_index))
            .or_insert_with(|| destination_and_time(network, directions, direction_index, ghost.0));
        *ghost = (destination, ghost.1 + time);
    }
    ghosts[0].1
}

pub fn first(input: &str) -> String {
    let (directions, network) = directions_and_network(input);
    destination_and_time(&network, directions, 0, "AAA")
        .1
        .to_string()
}

pub fn second(input: &str) -> String {
    let (directions, network) = directions_and_network(input);
    time_to_all_ghosts_at_destinations(&network, directions).to_string()
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 8;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 2);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 6);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 18673);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(2), 6);
    }

    // #[test]
    // fn second_input() {
    //     // This takes roughly three minutes.
    //     test_on_input(
    //         DAY,
    //         Puzzle::Second,
    //         Input::PuzzleInput,
    //         17_972_669_116_327_usize,
    //     );
    // }
}
