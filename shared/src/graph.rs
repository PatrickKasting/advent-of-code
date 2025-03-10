use std::hash::Hash;

use ahash::AHashMap;
use itertools::Itertools;

#[must_use]
#[expect(
    clippy::missing_panics_doc,
    reason = "all edges of original graph should be present"
)]
pub fn topological_sort<Vertex>(graph: &AHashMap<Vertex, Vec<Vertex>>) -> Option<Vec<Vertex>>
where
    Vertex: Copy + Eq + Ord + Hash,
{
    let mut indegrees: AHashMap<Vertex, usize> = AHashMap::new();
    for (&from, tos) in graph {
        indegrees.entry(from).or_default();
        for &to in tos {
            *indegrees.entry(to).or_default() += 1;
        }
    }
    let mut zero_indegrees = indegrees
        .iter()
        .filter(|&(_, &indegree)| indegree == 0)
        .map(|(&vertex, _)| vertex)
        .collect_vec();

    let mut topological_sort = vec![];
    let empty = vec![];
    while let Some(vertex) = zero_indegrees.pop() {
        topological_sort.push(vertex);
        for &to in graph.get(&vertex).unwrap_or(&empty) {
            let indegree = indegrees
                .get_mut(&to)
                .expect("all edges of original graph should be present");
            *indegree -= 1;
            if *indegree == 0 {
                zero_indegrees.push(to);
            }
        }
    }
    (topological_sort.len() == indegrees.len()).then_some(topological_sort)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topological_sort_exists() {
        let edges = [
            (75, 47),
            (75, 61),
            (75, 53),
            (75, 29),
            (75, 47),
            (47, 61),
            (47, 53),
            (47, 29),
            (75, 61),
            (47, 61),
            (61, 53),
            (61, 29),
            (53, 29),
            (53, 29),
        ];
        let mut graph: AHashMap<usize, Vec<usize>> = AHashMap::new();
        for (from, to) in edges {
            graph.entry(from).or_default().push(to);
        }
        let actual = topological_sort(&graph);
        let expected = Some(vec![75, 47, 61, 53, 29]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn topological_sort_does_not_exist() {
        let edges = [('A', 'B'), ('B', 'B')];
        let mut graph: AHashMap<char, Vec<char>> = AHashMap::new();
        for (from, to) in edges {
            graph.entry(from).or_default().push(to);
        }
        let actual = topological_sort(&graph);
        let expected = None;
        assert_eq!(actual, expected);
    }
}
