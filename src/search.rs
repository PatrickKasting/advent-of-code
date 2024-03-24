use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    hash::Hash,
    ops::Add,
};

pub struct DepthFirst<T: Copy + Eq + Hash> {
    explored: HashSet<T>,
}

impl<T: Copy + Eq + Hash> DepthFirst<T> {
    pub fn with_explored(explored: impl IntoIterator<Item = T>) -> Self {
        DepthFirst {
            explored: explored.into_iter().collect(),
        }
    }

    pub fn search<S: IntoIterator<Item = T>>(
        &mut self,
        mut successors: impl FnMut(T) -> S,
        from: T,
    ) {
        let mut frontier = vec![from];
        while let Some(element) = frontier.pop() {
            if self.explored.insert(element) {
                frontier.extend(successors(element));
            }
        }
    }

    pub fn explored(self) -> HashSet<T> {
        self.explored
    }
}

pub fn uniform_cost<
    State: Copy + Eq + Hash + Ord,
    Cost: Copy + Ord + Default + Add<Cost, Output = Cost>,
    Successors: IntoIterator<Item = (State, Cost)>,
>(
    initial: State,
    mut successors: impl FnMut(State) -> Successors,
    mut stop: impl FnMut(State) -> bool,
) -> Option<Cost> {
    let mut explored: HashSet<State> = HashSet::new();
    let mut frontier: BinaryHeap<(Reverse<Cost>, State)> =
        BinaryHeap::from([(Reverse(Cost::default()), initial)]);
    while let Some((Reverse(path_cost), state)) = frontier.pop() {
        if stop(state) {
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
