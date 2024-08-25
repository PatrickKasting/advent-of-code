use ahash::{AHashMap, AHashSet};
use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};

use shared::search::{shortest_path, Exploration};

type Apparatus<'input> = AHashMap<Component<'input>, AHashSet<Component<'input>>>;
type Connection<'input> = [Component<'input>; 2];
type Component<'input> = &'input str;

pub fn first(input: &str) -> String {
    let apparatus = apparatus(input);
    disconnected_group_sizes(&apparatus)
        .into_iter()
        .product::<usize>()
        .to_string()
}

pub fn second(_input: &str) -> String {
    panic!("there is no second part on the 25th");
}

fn disconnected_group_sizes(apparatus: &Apparatus) -> [usize; 2] {
    let mut rng = SmallRng::from_seed([0; 32]);
    loop {
        let terminals = apparatus.keys().copied().choose_multiple(&mut rng, 2);
        let (source, target) = (terminals[0], |component| component == terminals[1]);
        let mut paths: AHashSet<Connection> = AHashSet::new();
        let mut number_of_disjoint_paths = 0;
        loop {
            let successors = |from| {
                let paths = &paths;
                apparatus[&from]
                    .iter()
                    .copied()
                    .filter(move |&to| !paths.contains(&[from, to]))
            };
            let Some(path) = shortest_path(source, successors, target) else {
                return [0, 1].map(|index| group_size(apparatus, &paths, terminals[index]));
            };
            for connection in path.windows(2) {
                let [from, to] = [connection[0], connection[1]];
                if paths.contains(&[to, from]) {
                    paths.remove(&[to, from]);
                } else {
                    paths.insert([from, to]);
                }
            }
            number_of_disjoint_paths += 1;
            if number_of_disjoint_paths > 3 {
                break;
            }
        }
    }
}

fn group_size(apparatus: &Apparatus, paths: &AHashSet<Connection>, component: Component) -> usize {
    let successors = |from| {
        apparatus[&from]
            .iter()
            .copied()
            .filter(move |&to| !paths.contains(&[from, to]) && !paths.contains(&[to, from]))
    };
    let mut exploration = Exploration::new([]);
    exploration.explore(component, successors);
    exploration.explored().len()
}

fn apparatus(input: &str) -> Apparatus {
    let mut apparatus = AHashMap::new();
    let mut add_connection = |from, to| {
        apparatus
            .entry(from)
            .or_insert_with(AHashSet::new)
            .insert(to);
        apparatus
            .entry(to)
            .or_insert_with(AHashSet::new)
            .insert(from);
    };
    for line in input.lines() {
        let (from, tos) = line
            .split_once(": ")
            .expect("every line should contain a colon succeeded by a space");
        for to in tos.split(' ') {
            add_connection(from, to);
        }
    }
    apparatus
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 25;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 54);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 562_912);
    }
}
