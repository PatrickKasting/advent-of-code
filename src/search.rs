use std::{cmp::Reverse, collections::BinaryHeap, hash::Hash, mem, ops::Add};

use crate::HashSet;

pub struct Exploration<T: Copy + Eq + Hash>(HashSet<T>);

impl<State: Copy + Eq + Hash> Exploration<State> {
    pub fn new(explored: impl IntoIterator<Item = State>) -> Self {
        Exploration(explored.into_iter().collect())
    }

    pub fn explore<Successors: IntoIterator<Item = State>>(
        &mut self,
        source: State,
        mut successors: impl FnMut(State) -> Successors,
    ) {
        let mut frontier = vec![source];
        while let Some(state) = frontier.pop() {
            if self.0.insert(state) {
                frontier.extend(successors(state));
            }
        }
    }

    pub fn explored(self) -> HashSet<State> {
        self.0
    }
}

pub fn shortest_path_length<State: Copy + Eq + Hash, Successors: IntoIterator<Item = State>>(
    source: State,
    mut inspect: impl FnMut(State, usize),
    mut successors: impl FnMut(State) -> Successors,
    mut target: impl FnMut(State) -> bool,
) -> Option<usize> {
    let mut explored = HashSet::new();
    let mut current_ring = vec![];
    let mut next_ring = vec![source];
    let mut distance = 0;
    while !next_ring.is_empty() {
        mem::swap(&mut current_ring, &mut next_ring);
        while let Some(state) = current_ring.pop() {
            if explored.insert(state) {
                inspect(state, distance);
                if target(state) {
                    return Some(distance);
                }
                next_ring.extend(successors(state));
            }
        }
        distance += 1;
    }
    None
}

pub fn cheapest_path_cost<
    State: Copy + Eq + Hash + Ord,
    Cost: Copy + Ord + Default + Add<Cost, Output = Cost>,
    Successors: IntoIterator<Item = (State, Cost)>,
>(
    source: State,
    mut successors: impl FnMut(State) -> Successors,
    mut target: impl FnMut(State) -> bool,
) -> Option<Cost> {
    let mut explored: HashSet<State> = HashSet::new();
    let mut frontier: BinaryHeap<(Reverse<Cost>, State)> =
        BinaryHeap::from([(Reverse(Cost::default()), source)]);
    while let Some((Reverse(path_cost), state)) = frontier.pop() {
        if target(state) {
            return Some(path_cost);
        }
        if explored.contains(&state) {
            continue;
        }
        for (successor, step_cost) in successors(state) {
            frontier.push((Reverse(path_cost + step_cost), successor));
        }
        let inserted = explored.insert(state);
        debug_assert!(
            inserted,
            "set of explored states should never contain states from the frontier"
        );
    }
    None
}
