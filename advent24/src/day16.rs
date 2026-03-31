use std::{
    cmp::Reverse,
    collections::{BinaryHeap, hash_map::Entry},
};

use ahash::{AHashMap, AHashSet};
use shared::{
    grid::{Direction, EAST, Grid, ORTHOGONAL_DIRECTIONS, Position},
    search,
    vector::{RotationInTwoDimensions, Vector},
};

type Maze = Grid<u8>;

type State = (Position, Direction);
type Cost = usize;

pub fn first_answer(input: &str) -> String {
    let maze = Maze::from(input);
    best_path_cost(&maze).to_string()
}

pub fn second_answer(input: &str) -> String {
    let maze = Maze::from(input);
    best_paths(&maze).len().to_string()
}

fn best_path_cost(maze: &Maze) -> usize {
    search::minimum_path_cost(source(maze), successors(maze), target(maze))
        .expect("path from start tile to end tile should exist")
}

fn best_paths(maze: &Maze) -> AHashSet<Position> {
    let (source, successors, target) = (source(maze), successors(maze), target(maze));
    let mut predecessors: AHashMap<State, (Cost, Vec<State>)> = AHashMap::new();
    let mut frontier = BinaryHeap::from([(Reverse(0), source, None)]);
    let mut best_path_cost = None;
    while let Some((Reverse(path_cost), state, predecessor)) = frontier.pop() {
        if best_path_cost.is_some_and(|best_path_cost| best_path_cost < path_cost) {
            break;
        }
        match predecessors.entry(state) {
            Entry::Occupied(mut occupied_entry) => {
                if occupied_entry.get().0 == path_cost {
                    occupied_entry.get_mut().1.extend(predecessor);
                }
                continue;
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert((path_cost, Vec::from_iter(predecessor)));
            }
        }
        if target(state) {
            best_path_cost = Some(path_cost);
            continue;
        }
        for (successor, step_cost) in successors(state) {
            frontier.push((Reverse(path_cost + step_cost), successor, Some(state)));
        }
    }

    let mut best_paths = AHashSet::new();
    for direction in ORTHOGONAL_DIRECTIONS {
        self::path(&mut best_paths, &predecessors, (end_tile(maze), direction));
    }
    best_paths
}

fn path(
    path: &mut AHashSet<Position>,
    predecessors: &AHashMap<State, (Cost, Vec<State>)>,
    current @ (tile, _): State,
) {
    path.insert(tile);
    if let Some((_, preds)) = predecessors.get(&current) {
        for &predecessor in preds {
            self::path(path, predecessors, predecessor);
        }
    }
}

fn source(maze: &Maze) -> State {
    let start_tile = maze
        .find(|_, &element| element == b'S')
        .expect("maze should have a start tile")
        .0;
    (start_tile, EAST)
}

fn successors(maze: &Maze) -> impl Fn(State) -> Vec<(State, Cost)> {
    |(tile, direction)| {
        let mut successors = vec![
            ((tile, direction.left()), 1000),
            ((tile, direction.right()), 1000),
        ];
        let next_tile = tile.add(direction);
        if maze[next_tile] != b'#' {
            successors.push(((next_tile, direction), 1_usize));
        }
        successors
    }
}

fn target(maze: &Maze) -> impl Fn(State) -> bool {
    let end_tile = end_tile(maze);
    move |(tile, _)| tile == end_tile
}

fn end_tile(maze: &Grid<u8>) -> [isize; 2] {
    maze.find(|_, &element| element == b'E')
        .expect("maze should have an end tile")
        .0
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 16;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 7036);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 11048);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 135_512);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 45);
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 64);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 541);
    }
}
