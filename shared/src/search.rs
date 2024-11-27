use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BTreeMap, BTreeSet, BinaryHeap},
    hash::Hash,
    mem,
    ops::Add,
};

use ahash::{AHashMap, AHashSet};
use itertools::Itertools;

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

#[must_use]
pub fn injections<K, V>(possibilities: BTreeMap<K, BTreeSet<V>>) -> Vec<AHashMap<K, V>>
where
    K: Copy + Eq + Hash,
    V: Copy + Eq + Hash,
{
    let mut possibilities_descending = possibilities
        .into_iter()
        .sorted_unstable_by_key(|(_, values)| usize::MAX - values.len())
        .collect_vec();
    all_injections(&mut AHashSet::new(), &mut possibilities_descending)
}

fn all_injections<K, V>(
    invalid: &mut AHashSet<V>,
    possibilites: &mut Vec<(K, BTreeSet<V>)>,
) -> Vec<AHashMap<K, V>>
where
    K: Copy + Eq + Hash,
    V: Copy + Eq + Hash,
{
    if possibilites.is_empty() {
        return vec![AHashMap::new()];
    }

    let mut injections = vec![];
    let (key, values) = possibilites
        .pop()
        .expect("possibilities should not be empty");
    for &value in &values {
        if !invalid.insert(value) {
            continue;
        }
        for mut injection in all_injections(invalid, possibilites) {
            injection.insert(key, value);
            injections.push(injection);
        }
        invalid.remove(&value);
    }
    possibilites.push((key, values));
    injections
}

#[cfg(test)]
mod tests {
    use infrastructure::test;

    use super::*;

    #[test]
    fn assignment() {
        type Case<'case, 'values> = (&'case [(usize, &'values [char])], usize);

        let function = |possibilities: &[(usize, &[char])]| {
            let possibilities: BTreeMap<usize, BTreeSet<char>> = possibilities
                .iter()
                .map(|&(key, values)| (key, values.iter().copied().collect()))
                .collect();
            super::injections(possibilities).len()
        };
        let cases: [Case; 5] = [
            (
                &[
                    (1, &['a', 'b', 'c']),
                    (2, &['a', 'b', 'c']),
                    (3, &['a', 'b', 'c']),
                ],
                6,
            ),
            (
                &[
                    (2, &['b', 'c']),
                    (3, &['a', 'b', 'c']),
                    (5, &['a', 'b', 'c']),
                ],
                4,
            ),
            (&[(7, &['b', 'c']), (11, &['a', 'b', 'c']), (14, &['b'])], 1),
            (&[(4, &['b', 'c']), (16, &['c', 'b']), (64, &['b', 'c'])], 0),
            (&[], 1),
        ];
        test::cases(function, cases);
    }
}
