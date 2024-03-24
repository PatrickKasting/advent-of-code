use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::data_structures::grid::{Direction, Grid, Position};

type Map = Grid<char>;
type Graph = HashMap<Position, Vec<(usize, Position)>>;

fn start() -> Position {
    Position::new(0, 1)
}

fn goal(map: &Map) -> Position {
    Position::new(map.height() - 1, map.width() - 2)
}

fn slope_direction(char: char) -> Direction {
    match char {
        '^' => Direction::North,
        '>' => Direction::East,
        'v' => Direction::South,
        '<' => Direction::West,
        _ => panic!("slope should be '^', '>', 'v', or '<'"),
    }
}

fn successor_tiles(
    ignore_slopes: bool,
    map: &Grid<char>,
    from: Direction,
    to: Position,
) -> Vec<(Direction, Position)> {
    Direction::iter()
        .filter(|&direction| direction != from.opposite())
        .filter_map(|direction| {
            let neighbor = to.neighbor(direction);
            match map.get(neighbor) {
                None | Some(&'#') => None,
                Some(&'.') => Some((direction, neighbor)),
                Some(&slope) if ignore_slopes || slope_direction(slope) == direction => {
                    Some((direction, neighbor))
                }
                Some(_) => None,
            }
        })
        .collect_vec()
}

fn path_to_next_junction(
    ignore_slopes: bool,
    map: &Grid<char>,
    mut position: Position,
    mut direction: Direction,
) -> (usize, Position, impl Iterator<Item = Direction>) {
    position = position.neighbor(direction);
    let mut distance = 1;
    loop {
        let successors = successor_tiles(ignore_slopes, map, direction, position);
        if successors.len() == 1 {
            let (successor_direction, successor_position) = successors[0];
            direction = successor_direction;
            position = successor_position;
        } else {
            let successor_directions = successors.into_iter().map(|(direction, _)| direction);
            return (distance, position, successor_directions);
        }
        distance += 1;
    }
}

fn graph(ignore_slopes: bool, map: &Grid<char>) -> Graph {
    let mut graph = HashMap::new();
    let mut explored = HashSet::from([(start(), Direction::South)]);
    let mut frontier = vec![(start(), Direction::South)];
    while let Some((position, direction)) = frontier.pop() {
        let (distance, junction, successor_directions) =
            path_to_next_junction(ignore_slopes, map, position, direction);
        graph
            .entry(position)
            .or_insert(vec![])
            .push((distance, junction));
        for successor_direction in successor_directions {
            if explored.insert((junction, successor_direction)) {
                frontier.push((junction, successor_direction));
            }
        }
    }
    graph
}

fn maximum_distance(
    graph: &Graph,
    explored: &mut HashSet<Position>,
    from: Position,
    to: Position,
) -> Option<usize> {
    // add 'from' to 'explored' or return if already present
    if !explored.insert(from) {
        return None;
    }
    if from == to {
        explored.remove(&from); // remove 'from' to restore 'explored' to state before this call
        return Some(0);
    }
    let maximum_distance = graph[&from]
        .iter()
        .filter_map(|&(distance, successor)| {
            maximum_distance(graph, explored, successor, to)
                .map(|maximum_distance| maximum_distance + distance)
        })
        .max();
    explored.remove(&from); // remove 'from' to restore 'explored' to state before this call
    maximum_distance
}

fn longest_hike(input: &str, ignore_slopes: bool) -> usize {
    let map = Map::from(input);
    let graph = graph(ignore_slopes, &map);
    maximum_distance(&graph, &mut HashSet::new(), start(), goal(&map))
        .expect("at least one hike should lead to the goal")
}

pub fn first(input: &str) -> String {
    longest_hike(input, false).to_string()
}

pub fn second(input: &str) -> String {
    longest_hike(input, true).to_string()
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 23;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 94);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 2202);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 154);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 6226);
    }
}
