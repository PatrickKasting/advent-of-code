use std::cmp;

use easy_cast::Cast;
use itertools::Itertools;

use crate::{
    grid::{self, Direction, Grid, Position},
    vector::{Negation, Vector},
    HashMap, HashSet,
};

type Graph = HashMap<Position, Vec<(Position, Distance)>>;
type Distance = usize;
type Map = Grid<Tile>;
type Tile = char;

const START: Position = [0, 1];

pub fn first(input: &str) -> String {
    longest_hike(input, true).to_string()
}

pub fn second(input: &str) -> String {
    longest_hike(input, false).to_string()
}

fn longest_hike(input: &str, slopes: bool) -> Distance {
    let mut map = Map::from(input);
    if !slopes {
        map = map.map(|_, &tile| slope_to_path(tile));
    }
    let graph = graph(&map);
    let (last_junction, distance_from_last_junction_to_goal) = last_junction(&graph, goal(&map));
    let maximum_distance_to_last_junction = maximum_distance(&graph, START, last_junction);
    maximum_distance_to_last_junction + distance_from_last_junction_to_goal
}

fn maximum_distance(graph: &Graph, from: Position, to: Position) -> Distance {
    const STACK_MARKER: (Position, Distance) = ([0, 0], 0);
    let mut maximum_distance = Distance::MIN;
    let mut path = vec![];
    let mut frontier = vec![(from, 0)];
    while let Some((position, distance)) = frontier.pop() {
        if position == STACK_MARKER.0 {
            path.pop();
            continue;
        }

        if position == to {
            maximum_distance = cmp::max(maximum_distance, distance);
            continue;
        }

        path.push(position);
        frontier.push(STACK_MARKER);
        for &(successor, step_distance) in &graph[&position] {
            if !path.contains(&successor) {
                frontier.push((successor, distance + step_distance));
            }
        }
    }
    maximum_distance
}

fn graph(map: &Map) -> Graph {
    let mut explored = HashSet::from([(START, grid::SOUTH)]);
    let mut frontier = vec![(START, grid::SOUTH)];
    let mut graph = HashMap::new();
    while let Some((from, toward)) = frontier.pop() {
        if let Some((to, next_towards, distance)) = next_junction(map, from, toward) {
            graph.entry(from).or_insert(vec![]).push((to, distance));
            for next_toward in next_towards {
                if explored.insert((to, next_toward)) {
                    frontier.push((to, next_toward));
                }
            }
        }
    }
    graph
}

fn next_junction(
    map: &Map,
    from: Position,
    mut toward: Direction,
) -> Option<(Position, impl Iterator<Item = Direction>, Distance)> {
    let mut position = from.add(toward);

    if ![None, Some(toward)].contains(&slope(map[position])) {
        return None;
    }

    let mut distance = 1;
    let mut next_path_tiles = next_path_tiles(map, toward.neg(), position);
    while next_path_tiles.len() == 1 {
        (position, toward) = next_path_tiles[0];
        distance += 1;
        next_path_tiles = self::next_path_tiles(map, toward.neg(), position);
    }

    if ![None, Some(toward)].contains(&slope(map[position.sub(toward)])) {
        return None;
    }

    let next_towards = next_path_tiles.into_iter().map(|(_, direction)| direction);
    Some((position, next_towards, distance))
}

fn next_path_tiles(map: &Map, from: Direction, position: Position) -> Vec<(Position, Direction)> {
    grid::DIRECTIONS
        .into_iter()
        .filter(|&direction| direction != from)
        .filter_map(move |direction| {
            let neighbor = position.add(direction);
            (![None, Some(&'#')].contains(&map.get(neighbor))).then_some((neighbor, direction))
        })
        .collect_vec()
}

fn slope(tile: Tile) -> Option<Direction> {
    match tile {
        '^' => Some(grid::NORTH),
        '>' => Some(grid::EAST),
        'v' => Some(grid::SOUTH),
        '<' => Some(grid::WEST),
        '.' => None,
        _ => panic!("only walkable tiles should be checked for slopes"),
    }
}

fn last_junction(graph: &Graph, goal: Position) -> (Position, Distance) {
    let last_junction = graph.iter().find_map(|(&position, successors)| {
        successors
            .iter()
            .find(|&&(successor, _)| successor == goal)
            .map(|&(_, distance)| (position, distance))
    });
    last_junction.expect("single junction should connect to goal")
}

fn slope_to_path(tile: Tile) -> Tile {
    if ['^', '>', 'v', '<'].contains(&tile) {
        '.'
    } else {
        tile
    }
}

fn goal(map: &Map) -> Position {
    [(map.height() - 1).cast(), (map.width() - 2).cast()]
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
