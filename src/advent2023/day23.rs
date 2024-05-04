use std::collections::{HashMap, HashSet};

use easy_cast::Conv;
use itertools::Itertools;

use crate::{
    data_structures::grid::{self, Coordinate, Direction, Grid, Position},
    vector::{Addition, Negation},
};

type Graph = HashMap<Position, Vec<(Distance, Position)>>;
type Distance = usize;
type Map = Grid<char>;

pub fn first(input: &str) -> String {
    longest_hike(input, false).to_string()
}

pub fn second(input: &str) -> String {
    longest_hike(input, true).to_string()
}

fn longest_hike(input: &str, ignore_slopes: bool) -> Distance {
    let map = Map::from(input);
    let graph = graph(ignore_slopes, &map);
    maximum_distance(&graph, &mut HashSet::new(), start(), goal(&map))
        .expect("at least one hike should lead to the goal")
}

fn graph(ignore_slopes: bool, map: &Map) -> Graph {
    let mut graph = HashMap::new();
    let mut explored = HashSet::from([(start(), grid::SOUTH)]);
    let mut frontier = vec![(start(), grid::SOUTH)];
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

fn path_to_next_junction(
    ignore_slopes: bool,
    map: &Map,
    mut position: Position,
    mut direction: Direction,
) -> (Distance, Position, impl Iterator<Item = Direction>) {
    position = position.add(direction);
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

fn successor_tiles(
    ignore_slopes: bool,
    map: &Map,
    from: Direction,
    to: Position,
) -> Vec<(Direction, Position)> {
    grid::directions()
        .filter(|&direction| direction != from.neg())
        .filter_map(|direction| {
            let neighbor = to.add(direction);
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

fn slope_direction(char: char) -> Direction {
    match char {
        '^' => grid::NORTH,
        '>' => grid::EAST,
        'v' => grid::SOUTH,
        '<' => grid::WEST,
        _ => panic!("slope should be '^', '>', 'v', or '<'"),
    }
}

fn maximum_distance(
    graph: &Graph,
    explored: &mut HashSet<Position>,
    from: Position,
    to: Position,
) -> Option<Distance> {
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

fn start() -> Position {
    [0, 1]
}

fn goal(map: &Map) -> Position {
    [
        Coordinate::conv(map.height()) - 1,
        Coordinate::conv(map.width()) - 2,
    ]
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
