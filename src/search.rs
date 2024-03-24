use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    hash::Hash,
    ops::Add,
};

pub struct Exploration<T: Copy + Eq + Hash>(HashSet<T>);

impl<State: Copy + Eq + Hash> Exploration<State> {
    pub fn new(explored: impl IntoIterator<Item = State>) -> Self {
        Exploration(explored.into_iter().collect())
    }

    pub fn explore<S: IntoIterator<Item = State>>(
        &mut self,
        from: State,
        mut successors: impl FnMut(State) -> S,
    ) {
        let mut frontier = vec![from];
        while let Some(element) = frontier.pop() {
            if self.0.insert(element) {
                frontier.extend(successors(element));
            }
        }
    }

    pub fn explored(self) -> HashSet<State> {
        self.0
    }
}

pub fn shortest_path_cost<
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
