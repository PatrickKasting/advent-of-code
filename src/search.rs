use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    hash::Hash,
    ops::Add,
};

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
        debug_assert!(
            explored.insert(state),
            "set of explored states should never contain states from the frontier"
        );
    }
    None
}
