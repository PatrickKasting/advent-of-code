use std::{
    cmp,
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

#[derive(Debug, Clone)]
struct FlowNetwork<N> {
    arcs: HashMap<N, HashSet<N>>,
    reverse_arcs: HashMap<N, HashSet<N>>,
    flows: HashMap<(N, N), (Number, Number)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum FlowDirection {
    Forward,
    Backward,
}

type Predecessors<N> = HashMap<N, (N, FlowDirection, Number)>;
type Number = usize;

pub fn maximum_flow<N: Debug + Copy + Eq + Hash>(
    nodes: impl IntoIterator<Item = N>,
    arcs: impl IntoIterator<Item = (N, Number, N)>,
    source: N,
    sink: N,
) -> (Number, (HashSet<N>, HashSet<N>)) {
    debug_assert_ne!(source, sink, "source and sink should differ");
    let mut flow_network = flow_network(nodes, arcs);
    let mut flow = 0;
    while let Some(predecessors) = augmenting_path(&flow_network, source, sink) {
        let additional_flow = additional_flow(&predecessors, source, sink);
        increase_flow(
            &mut flow_network.flows,
            &predecessors,
            additional_flow,
            source,
            sink,
        );
        flow += additional_flow;
    }
    (flow, cut(&flow_network, source))
}

fn flow_network<N: Debug + Copy + Eq + Hash>(
    nodes: impl IntoIterator<Item = N>,
    arcs: impl IntoIterator<Item = (N, Number, N)>,
) -> FlowNetwork<N> {
    let mut forward_arcs: HashMap<N, HashSet<N>> = nodes
        .into_iter()
        .map(|node| (node, HashSet::new()))
        .collect();
    let mut reverse_arcs: HashMap<N, HashSet<N>> = forward_arcs.clone();
    let mut flows = HashMap::new();
    for (from, capacity, to) in arcs {
        debug_assert!(
            forward_arcs.contains_key(&to),
            "flow network should contain 'to' node"
        );
        let is_new_edge = forward_arcs
            .get_mut(&from)
            .expect("flow network should contain 'from' node")
            .insert(to);
        debug_assert!(is_new_edge, "edge should occur only once");
        reverse_arcs
            .get_mut(&to)
            .expect("flow network should contain 'from' node")
            .insert(from);
        flows.insert((from, to), (0, capacity));
    }
    FlowNetwork {
        arcs: forward_arcs,
        reverse_arcs,
        flows,
    }
}

fn augmenting_path<N: Debug + Copy + Eq + Hash>(
    flow_network: &FlowNetwork<N>,
    source: N,
    sink: N,
) -> Option<Predecessors<N>> {
    // Breadth-first search
    let mut predecessors: Predecessors<N> = Predecessors::new();
    let mut frontier = VecDeque::from([source]);
    while let Some(from) = frontier.pop_front() {
        // Destination reached?
        if from == sink {
            return Some(predecessors);
        }

        // Forward edges
        for &to in &flow_network.arcs[&from] {
            if let Entry::Vacant(predecessor) = predecessors.entry(to) {
                let (flow, capacity) = flow_network.flows[&(from, to)];
                let remaining_capacity = capacity - flow;
                if remaining_capacity > 0 {
                    predecessor.insert((from, FlowDirection::Forward, remaining_capacity));
                    frontier.push_back(to);
                }
            }
        }

        // Backward edges
        for &to in &flow_network.reverse_arcs[&from] {
            if let Entry::Vacant(predecessor) = predecessors.entry(to) {
                let (flow, _) = flow_network.flows[&(to, from)];
                if flow > 0 {
                    predecessor.insert((from, FlowDirection::Backward, flow));
                    frontier.push_back(to);
                }
            }
        }
    }
    None
}

fn additional_flow<N: Debug + Copy + Eq + Hash>(
    predecessors: &Predecessors<N>,
    source: N,
    sink: N,
) -> Number {
    let mut additional_flow = Number::MAX;
    let mut current = sink;
    while current != source {
        let (predecessor, _, flow) = predecessors[&current];
        additional_flow = cmp::min(additional_flow, flow);
        current = predecessor;
    }
    additional_flow
}

fn increase_flow<N: Debug + Copy + Eq + Hash>(
    flows: &mut HashMap<(N, N), (Number, Number)>,
    predecessors: &Predecessors<N>,
    additional_flow: Number,
    source: N,
    sink: N,
) {
    let mut current = sink;
    while current != source {
        let (predecessor, direction, _) = predecessors[&current];
        let (arc, next_flow): (_, Box<dyn Fn(Number, Number) -> Number>) = match direction {
            FlowDirection::Forward => ((predecessor, current), Box::new(|lhs, rhs| lhs + rhs)),
            FlowDirection::Backward => ((current, predecessor), Box::new(|lhs, rhs| lhs - rhs)),
        };
        let (flow, capacity) = flows
            .get_mut(&arc)
            .expect("forward arc should be in the flow data structures");
        *flow = next_flow(*flow, additional_flow);
        debug_assert!(*flow <= *capacity, "flow should not exceed capacity");
        current = predecessor;
    }
}

fn cut<N: Debug + Copy + Eq + Hash>(
    flow_network: &FlowNetwork<N>,
    source: N,
) -> (HashSet<N>, HashSet<N>) {
    let mut source_subset = HashSet::from([source]);
    let mut frontier = vec![source];
    while let Some(current) = frontier.pop() {
        for &successor in &flow_network.arcs[&current] {
            let (flow, capacity) = flow_network.flows[&(current, successor)];
            if flow < capacity && source_subset.insert(successor) {
                frontier.push(successor);
            }
        }
    }

    let sink_subset = flow_network
        .arcs
        .keys()
        .copied()
        .filter(|node| !source_subset.contains(node))
        .collect();
    (source_subset, sink_subset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_network() {
        let nodes = 'A'..='G';
        let arcs = [
            ('A', 3, 'B'),
            ('A', 3, 'D'),
            ('B', 4, 'C'),
            ('C', 3, 'A'),
            ('C', 1, 'D'),
            ('C', 2, 'E'),
            ('D', 2, 'E'),
            ('D', 6, 'F'),
            ('E', 1, 'B'),
            ('E', 1, 'G'),
            ('F', 9, 'G'),
        ];
        let (maximum_flow, cut) = maximum_flow(nodes, arcs, 'A', 'G');
        assert_eq!(maximum_flow, 5);
        assert_eq!(cut.0, HashSet::from(['A', 'B', 'C', 'E']));
        assert_eq!(cut.1, HashSet::from(['D', 'F', 'G']));
    }

    #[test]
    fn adverserial_network() {
        let nodes = 'A'..='D';
        let arcs = [
            ('A', 1_000_000_000, 'B'),
            ('A', 1_000_000_000, 'C'),
            ('B', 1, 'C'),
            ('B', 1_000_000_000, 'D'),
            ('C', 1_000_000_000, 'D'),
        ];
        let (maximum_flow, _) = maximum_flow(nodes, arcs, 'A', 'D');
        assert_eq!(maximum_flow, 2_000_000_000);
    }

    #[test]
    fn cyclic_network() {
        let nodes = ['s', '1', '2', '3', '4', 't'];
        let arcs = [
            ('s', 10, '1'),
            ('s', 10, '2'),
            ('1', 2, '2'),
            ('1', 4, '3'),
            ('1', 8, '4'),
            ('2', 9, '4'),
            ('3', 10, 't'),
            ('4', 6, '3'),
            ('4', 10, 't'),
        ];
        let reverse_arcs = arcs
            .into_iter()
            .map(|(from, capacity, to)| (to, capacity, from));
        let loops = nodes.into_iter().map(|node| (node, 1000, node));
        let arcs = arcs.into_iter().chain(reverse_arcs).chain(loops);
        let (maximum_flow, _) = maximum_flow(nodes, arcs, 's', 't');
        assert_eq!(maximum_flow, 20);
    }
}
