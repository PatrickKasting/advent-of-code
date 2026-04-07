use std::{
    fmt::{Debug, DebugSet},
    hash::Hash,
};

use ahash::AHashMap;

pub struct Trie<T> {
    root: Node<T>,
}

struct Node<T> {
    is_end: bool,
    children: AHashMap<T, Node<T>>,
}

impl<T> Trie<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            root: Node {
                is_end: false,
                children: AHashMap::new(),
            },
        }
    }
}

impl<T> Default for Trie<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + Eq + Hash> Trie<T> {
    pub fn insert(&mut self, sequence: impl IntoIterator<Item = T>) -> bool {
        let mut node = &mut self.root;
        for element in sequence {
            node = node.children.entry(element).or_insert_with(|| Node {
                is_end: false,
                children: AHashMap::new(),
            });
        }
        let newly_inserted = !node.is_end;
        node.is_end = true;
        newly_inserted
    }

    pub fn contains(&self, sequence: impl IntoIterator<Item = T>) -> bool {
        let mut node = &self.root;
        for element in sequence {
            match node.children.get(&element) {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_end
    }
}

impl<T: Debug + Eq + Hash> Debug for Trie<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_node<'element, T: Debug + Eq + Hash>(
            debug_set: &mut DebugSet,
            sequence: &mut Vec<&'element T>,
            node: &'element Node<T>,
        ) {
            if node.is_end {
                debug_set.entry(&sequence);
            }
            for (element, child) in &node.children {
                sequence.push(element);
                fmt_node(debug_set, sequence, child);
                sequence.pop();
            }
        }

        let mut debug_set = f.debug_set();
        fmt_node(&mut debug_set, &mut vec![], &self.root);
        debug_set.finish()
    }
}

#[cfg(test)]
mod tests {
    use ahash::AHashSet;
    use rand::{RngExt, distr::Alphabetic};

    use super::*;

    #[test]
    fn behaves_as_set() {
        let mut expected: AHashSet<String> = AHashSet::new();
        let mut actual: Trie<char> = Trie::new();
        for _ in 0..1_000_000 {
            let string = random_string(4);
            let insert = rand::rng().random_bool(0.9);
            if insert {
                assert_eq!(
                    expected.insert(string.clone()),
                    actual.insert(string.chars()),
                    "expected and actual should agree on insertion of '{string}'"
                );
            } else {
                assert_eq!(
                    expected.contains(&string),
                    actual.contains(string.chars()),
                    "expected and actual should agree on membership of '{string}'"
                );
            }
        }
    }

    fn random_string(max_len: usize) -> String {
        let len = rand::rng().random_range(..=max_len);
        rand::rng()
            .sample_iter(&Alphabetic)
            .take(len)
            .map(char::from)
            .collect()
    }
}
