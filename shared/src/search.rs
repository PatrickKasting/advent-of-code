use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap},
    hash::Hash,
    mem,
    ops::Add,
};

use ahash::{AHashMap, AHashSet};

pub struct Exploration<T: Copy + Eq + Hash>(AHashSet<T>);

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

    #[must_use]
    pub fn explored(self) -> AHashSet<State> {
        self.0
    }
}

pub fn shortest_path_length<State, Successors>(
    source: State,
    mut successors: impl FnMut(State) -> Successors,
    mut target: impl FnMut(State) -> bool,
) -> Option<usize>
where
    State: Copy + Eq + Hash,
    Successors: IntoIterator<Item = State>,
{
    let mut explored = AHashSet::new();
    let mut current_ring = vec![];
    let mut next_ring = vec![source];
    let mut distance = 0;
    while !next_ring.is_empty() {
        mem::swap(&mut current_ring, &mut next_ring);
        while let Some(state) = current_ring.pop() {
            if target(state) {
                return Some(distance);
            }
            if explored.insert(state) {
                next_ring.extend(successors(state));
            }
        }
        distance += 1;
    }
    None
}

pub fn shortest_path<State, Successors>(
    source: State,
    mut successors: impl FnMut(State) -> Successors,
    mut target: impl FnMut(State) -> bool,
) -> Option<Vec<State>>
where
    State: Copy + Eq + Hash,
    Successors: IntoIterator<Item = State>,
{
    let mut predecessors = AHashMap::from([(source, None)]);
    let mut current_ring = vec![];
    let mut next_ring = vec![source];
    while !next_ring.is_empty() {
        mem::swap(&mut current_ring, &mut next_ring);
        while let Some(state) = current_ring.pop() {
            for successor in successors(state) {
                match predecessors.entry(successor) {
                    Entry::Occupied(_) => (),
                    Entry::Vacant(entry) => {
                        entry.insert(Some(state));
                        if target(successor) {
                            return Some(path(&predecessors, successor));
                        }
                        next_ring.push(successor);
                    }
                }
            }
        }
    }
    None
}

fn path<State: Copy + Eq + Hash>(
    predecessors: &AHashMap<State, Option<State>>,
    end: State,
) -> Vec<State> {
    let mut current = end;
    let mut path = vec![end];
    while let Some(predecessor) = predecessors[&current] {
        path.push(predecessor);
        current = predecessor;
    }
    path.reverse();
    path
}

pub fn distances<State, Successors>(
    source: State,
    mut successors: impl FnMut(State) -> Successors,
) -> AHashMap<State, usize>
where
    State: Copy + Eq + Hash,
    Successors: IntoIterator<Item = State>,
{
    let mut distances = AHashMap::new();
    let mut current_ring = vec![];
    let mut next_ring = vec![source];
    let mut distance = 0;
    while !next_ring.is_empty() {
        mem::swap(&mut current_ring, &mut next_ring);
        while let Some(state) = current_ring.pop() {
            match distances.entry(state) {
                Entry::Occupied(_) => (),
                Entry::Vacant(entry) => {
                    entry.insert(distance);
                    next_ring.extend(successors(state));
                }
            }
        }
        distance += 1;
    }
    distances
}

pub fn minimum_path_cost<State, Cost, Successors>(
    source: State,
    mut successors: impl FnMut(State) -> Successors,
    mut target: impl FnMut(State) -> bool,
) -> Option<Cost>
where
    State: Copy + Eq + Hash + Ord,
    Cost: Copy + Ord + Default + Add<Cost, Output = Cost>,
    Successors: IntoIterator<Item = (State, Cost)>,
{
    let mut explored: AHashSet<State> = AHashSet::new();
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
